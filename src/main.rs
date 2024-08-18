mod r#cars;
mod r#track;

use crate::track::TrackName;
use anyhow::Result;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use crossterm::cursor::{MoveToNextLine, MoveToPreviousLine};
use crossterm::style::{Attribute, SetAttribute};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use discord_webhook2::message::Message;
use discord_webhook2::webhook::DiscordWebhook;
use dotenv::dotenv;
use r#cars::Car;
use simetry::assetto_corsa_competizione;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::io::{stdout, Stdout, Write};
use std::time::Duration;

#[derive(sqlx::FromRow, Clone, Debug)]
struct Driver {
    id: i64,
    name: String,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Clone, Debug)]
struct TrackRow {
    id: i64,
    name: String,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Clone, Debug)]
struct CarRow {
    id: i64,
    name: String,
    category: String, // Store category as a string
}

trait LapTime {
    fn lap_time_ms(&self) -> i64;
    fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
}

#[derive(sqlx::FromRow, Clone)]
struct BestLap {
    driver_id: i64,
    track_id: i64,
    created_at: chrono::DateTime<chrono::Utc>,
    lap_time_ms: i64,
    car_id: i64,
}

impl LapTime for BestLapWithDriver {
    fn lap_time_ms(&self) -> i64 {
        self.lap_time_ms
    }

    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Clone)]
struct BestLapWithDriver {
    id: i64,
    driver_id: i64,
    track_id: i64,
    created_at: chrono::DateTime<chrono::Utc>,
    lap_time_ms: i64,
    driver_name: String,
    car_id: i64,
}

#[derive(Clone)]
struct MyLapAndBestLap {
    mine: Option<BestLapWithDriver>,
    overall: Option<BestLapWithDriver>,
}

impl LapTime for BestLap {
    fn lap_time_ms(&self) -> i64 {
        self.lap_time_ms
    }

    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}

#[derive(Clone)]
struct BestLaps {
    car: MyLapAndBestLap,
    category: MyLapAndBestLap,
}

const LINE_LENGTH: usize = 128;

fn pad_string(input: String) -> String {
    if input.len() > LINE_LENGTH {
        input[..LINE_LENGTH].to_string()
    } else {
        let mut padded = String::from(input.clone());
        padded.extend(std::iter::repeat(' ').take(LINE_LENGTH - input.len()));
        padded
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*env::var("DATABASE_URL").expect("DATABASE_URL is not set"))
        .await?;

    let discord_webhook_url = env::var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOK is not set");
    let discord_webhook: DiscordWebhook =
        DiscordWebhook::new(discord_webhook_url).expect("Invalid webhook");
    let driver_name = env::var("DRIVER_NAME").expect("DRIVER_NAME is not set");
    let driver = sqlx::query_as!(
        Driver,
        "INSERT INTO driver (name) VALUES ($1) ON CONFLICT (name) DO UPDATE set name=$1 RETURNING *",
        driver_name
    )
    .fetch_one(&pool)
    .await?;

    loop {
        let mut printer = stdout();
        printer.execute(Clear(ClearType::All))?;
        printer
            .execute(MoveToPreviousLine(5))?
            .execute(Print(pad_string(format!(
                "Welcome {}, start a session to begin...",
                driver.name,
            ))))?
            .flush()?;

        let mut client = assetto_corsa_competizione::Client::connect(Duration::from_secs(1)).await;
        let track_name = client.static_data().track.clone();
        let track: TrackName = track_name.parse().expect("Unknown track");

        let track_row = sqlx::query_as::<_, TrackRow>(
            "INSERT INTO track (name) VALUES ($1) ON CONFLICT (name) DO UPDATE set name=$1 RETURNING *",
        )
        .bind(&track_name)
        .fetch_one(&pool)
        .await?;

        let car_model = client.static_data().car_model.clone();
        let car = Car::from_str(car_model.as_str()).expect("Unknown car model");
        let car_row = sqlx::query_as!(
            CarRow,
            "INSERT INTO car (name, category) VALUES ($1, $2) ON CONFLICT (name) DO UPDATE set name=$1 RETURNING *",
            &car_model,
            &car.category.to_string()
        )
            .fetch_one(&pool)
            .await?;

        printer
            .execute(cursor::MoveToColumn(0))?
            .execute(SetAttribute(Attribute::Bold))?
            .execute(SetAttribute(Attribute::Underlined))?
            .execute(Print(pad_string(format!(
                "{} ({}) on {}",
                car.name, car.category, track
            ))))?
            .execute(ResetColor)?
            .execute(SetAttribute(Attribute::Reset))?
            .flush()?;

        let mut lap_number = 0;
        let mut best_laps =
            refresh_laps(&pool, &driver, &track_row, &car_row, true, printer.by_ref()).await?;

        while let Some(sim_state) = client.next_sim_state().await {
            let mut refresh = false;
            if sim_state.graphics.completed_laps.gt(&lap_number) {
                lap_number = sim_state.graphics.completed_laps;
                refresh = true;

                if sim_state.graphics.lap_timing.best.millis < i32::MAX {
                    if best_laps.car.mine.clone().is_none()
                        || (best_laps.car.mine.clone().is_some_and(|t| {
                            sim_state.graphics.lap_timing.best.millis < t.lap_time_ms as i32
                        }))
                    {
                        let new_best_time = BestLap {
                            driver_id: driver.id,
                            track_id: track_row.id,
                            created_at: chrono::Utc::now(),
                            lap_time_ms: sim_state.graphics.lap_timing.best.millis as i64,
                            car_id: car_row.id,
                        };

                        sqlx::query!(
                "INSERT INTO best_lap (driver_id, track_id, created_at, lap_time_ms, car_id) VALUES ($1, $2, $3, $4, $5)",
                new_best_time.driver_id,
                new_best_time.track_id,
                new_best_time.created_at,
                new_best_time.lap_time_ms,
                new_best_time.car_id
            ).execute(&pool).await?;

                        discord_webhook
                            .send(&Message::new(|m| {
                                m.content(format!(
                                    "{} has a new best lap of {} on {} ({}) in {}",
                                    driver.name,
                                    format_lap_time(Some(new_best_time.clone()), false),
                                    car.name,
                                    car.category,
                                    track
                                ))
                            }))
                            .await?;
                        refresh = true;
                    }
                }
            }

            if refresh {
                best_laps = refresh_laps(
                    &pool,
                    &driver,
                    &track_row,
                    &car_row,
                    false,
                    printer.by_ref(),
                )
                .await?;
            }
        }
    }
}

async fn refresh_laps(
    pool: &Pool<Postgres>,
    driver: &Driver,
    track: &TrackRow,
    car: &CarRow,
    is_init: bool,
    printer: &mut Stdout,
) -> Result<BestLaps> {
    let car_records = sqlx::query_as::<_, BestLapWithDriver>(
        r#"SELECT best_lap.id,
       best_lap.track_id,
       best_lap.driver_id,
       best_lap.lap_time_ms,
       best_lap.created_at,
       best_lap.car_id,
       d."name" as driver_name,
       c.name     as car_name,
       c.category as car_category
       from best_lap
         INNER JOIN public.driver d on d.id = best_lap.driver_id
         INNER JOIN public.car c on c.id = best_lap.car_id
       WHERE track_id = $1 AND c.id = $2
       ORDER BY lap_time_ms ASC"#,
    )
    .bind(track.id)
    .bind(car.id)
    .fetch_all(pool)
    .await?;

    let category_records = sqlx::query_as::<_, BestLapWithDriver>(
        r#"SELECT best_lap.id,
       best_lap.track_id,
       best_lap.driver_id,
       best_lap.lap_time_ms,
       best_lap.created_at,
       best_lap.car_id,
       d."name" as driver_name,
       c.name     as car_name,
       c.category as car_category
       from best_lap
         INNER JOIN public.driver d on d.id = best_lap.driver_id
         INNER JOIN public.car c on c.id = best_lap.car_id
       WHERE track_id = $1 AND c.category = $2
       ORDER BY lap_time_ms ASC"#,
    )
    .bind(track.id)
    .bind(car.category.to_string())
    .fetch_all(pool)
    .await?;

    let my_best_lap_for_car = car_records
        .iter()
        .find(|r| r.driver_id == driver.id)
        .cloned();
    let my_best_lap_for_category = category_records
        .iter()
        .find(|r| r.driver_id == driver.id)
        .cloned();

    let best_overall_for_car = car_records.first().cloned();
    let best_overall_for_category = category_records.first().cloned();

    let result = BestLaps {
        car: MyLapAndBestLap {
            mine: my_best_lap_for_car,
            overall: best_overall_for_car,
        },
        category: MyLapAndBestLap {
            mine: my_best_lap_for_category,
            overall: best_overall_for_category,
        },
    };
    log_laps(result.clone(), !is_init, printer)?;
    Ok(result.clone())
}

fn format_lap_time<T: LapTime>(lap_time: Option<T>, with_relative_set_at: bool) -> String {
    let time = lap_time
        .map(|t| {
            let lap_duration = Duration::from_millis(t.lap_time_ms() as u64);
            format!(
                "{}:{}:{} set {}",
                lap_duration.as_secs() / 60,
                lap_duration.as_secs() % 60,
                lap_duration.subsec_millis(),
                &*HumanTime::from(t.created_at()).to_text_en(Accuracy::Rough, Tense::Past)
            )
        })
        .unwrap_or("None".to_string());
    if with_relative_set_at {
        time
    } else {
        time.split(" set ").next().unwrap().to_string()
    }
}

fn log_laps(laps: BestLaps, refresh: bool, printer: &mut Stdout) -> Result<()> {
    let is_my_lap_for_car_fastest = laps.car.clone().mine.is_some_and(|mine| {
        laps.car
            .clone()
            .overall
            .is_some_and(|overall| mine.id == overall.id)
    });

    if refresh {
        printer.execute(MoveToPreviousLine(2))?;
    }

    if is_my_lap_for_car_fastest {
        printer
            .execute(MoveToNextLine(1))?
            .execute(SetForegroundColor(Color::Green))?
            .execute(Print(pad_string(format!(
                "Your lap of {} is the fastest for this car!",
                format_lap_time(laps.clone().car.mine, true),
            ))))?
            .execute(ResetColor)?
            .execute(SetAttribute(Attribute::Reset))?;
    } else if laps.car.clone().mine.is_some() {
        printer
            .execute(MoveToNextLine(1))?
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print(pad_string(format!(
                "For this car: Personal best: {}, Best overall: {}",
                format_lap_time(laps.clone().car.mine, true),
                format_lap_time(laps.car.overall, true),
            ))))?
            .execute(ResetColor)?
            .execute(SetAttribute(Attribute::Reset))?;
    } else if laps.car.overall.is_some() {
        printer
            .execute(MoveToNextLine(1))?
            .execute(SetForegroundColor(Color::Red))?
            .execute(Print(pad_string(format!(
                "You have not set a lap time for this car, Best overall: {}",
                format_lap_time(laps.car.overall, true),
            ))))?
            .execute(ResetColor)?
            .execute(SetAttribute(Attribute::Reset))?;
    } else {
        printer
            .execute(MoveToNextLine(1))?
            .execute(SetForegroundColor(Color::Magenta))?
            .execute(Print(pad_string(
                "There are no lap times for this car".to_string(),
            )))?
            .execute(ResetColor)?
            .execute(SetAttribute(Attribute::Reset))?;
    }

    let is_my_lap_for_category_fastest = laps.category.clone().mine.is_some_and(|mine| {
        laps.category
            .clone()
            .overall
            .is_some_and(|overall| mine.id == overall.id)
    });

    if is_my_lap_for_category_fastest {
        printer
            .execute(MoveToNextLine(1))?
            .execute(SetForegroundColor(Color::Green))?
            .execute(Print(pad_string(format!(
                "Your lap of {} is the fastest for this category!",
                format_lap_time(laps.category.mine, true)
            ))))?
            .execute(ResetColor)?
            .execute(SetAttribute(Attribute::Reset))?;
    } else if laps.category.clone().mine.is_some() {
        printer
            .execute(MoveToNextLine(1))?
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print(pad_string(format!(
                "For this category: Personal best: {}, Best overall: {}",
                format_lap_time(laps.category.mine, true),
                format_lap_time(laps.category.overall, true),
            ))))?
            .execute(ResetColor)?
            .execute(SetAttribute(Attribute::Reset))?;
    } else if laps.category.overall.is_some() {
        printer
            .execute(MoveToNextLine(1))?
            .execute(SetForegroundColor(Color::Red))?
            .execute(Print(pad_string(format!(
                "You have not set a lap time for this category, Best overall: {}",
                format_lap_time(laps.category.overall, true),
            ))))?
            .execute(ResetColor)?
            .execute(SetAttribute(Attribute::Reset))?;
    } else {
        printer
            .execute(MoveToNextLine(1))?
            .execute(SetForegroundColor(Color::Magenta))?
            .execute(Print(pad_string(
                "There are no lap times for this category".to_string(),
            )))?
            .execute(ResetColor)?
            .execute(SetAttribute(Attribute::Reset))?;
    }
    Ok(())
}
