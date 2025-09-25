#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod r#cars;
mod r#track;
use crate::track::TrackName;
mod lap_formatter;
mod notifications;
mod persistence;
mod telemetry;

use crate::cars::Car;
use crate::lap_formatter::{diff_lap_time, format_lap_time, pad_lap_segment};
use crate::notifications::{DiscordNotifier, Notifier};
use crate::persistence::{
    BestLapData, BestLaps, CarRow, Driver, LapTime, MyLapAndBestLap, Repository, TrackRow,
};
use crate::telemetry::TelemetryFactory;
use crate::telemetry::acc::AccFactory;
use dotenv::dotenv;
use eframe::{App, Frame, NativeOptions, egui};
use simetry::assetto_corsa_competizione::{SessionType, Status};
use std::{
    env,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::runtime::Runtime;

#[derive(Clone)]
struct LapTimeWithDiff {
    driver: String,
    time: String,
    diff_last: String,
    diff_mine: String,
}

struct UiState {
    status: String,
    driver_name: String,
    track_name: String,
    car_model: String,
    category_name: String,
    laps_run: i32,
    last_lap: Option<String>,
    session_best: Option<String>,
    best_car: Option<LapTimeWithDiff>,
    best_category: Option<LapTimeWithDiff>,
    info_log: Vec<String>,
    last_update: Option<chrono::DateTime<chrono::Local>>,
}

struct MyApp {
    rt: Arc<Runtime>,
    state: Arc<Mutex<UiState>>,
}

impl MyApp {
    fn new(rt: Arc<Runtime>, state: Arc<Mutex<UiState>>) -> Self {
        Self { rt, state }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            let st = self.state.lock().unwrap();
            ui.columns(2, |columns| {
                columns[0].vertical(|ui| {
                    ui.label(format!("Driver: {}", st.driver_name));
                    ui.label(format!("Status: {}", st.status));
                });
                if !st.track_name.eq("—") {
                    columns[1].vertical(|ui| {
                        ui.label(format!("Track: {}", st.track_name));
                        ui.label(format!(
                            "Category: {} Model {}",
                            st.category_name, st.car_model
                        ));
                    });
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let st = self.state.lock().unwrap();
            ui.columns(2, |columns| {
                columns[0].vertical(|ui| {
                    ui.heading(format!("Best time for {}", st.category_name.clone()));
                    ui.label(format!(
                        "Driver: {}",
                        st.best_category
                            .clone()
                            .map(|ltwd| ltwd.driver)
                            .unwrap_or("—".into())
                    ));
                    ui.label(format!(
                        "Time: {}",
                        st.best_category
                            .clone()
                            .map(|ltwd| ltwd.time)
                            .unwrap_or("—".into())
                    ));
                    ui.label(format!(
                        "Difference to mine: {}",
                        st.best_category
                            .clone()
                            .map(|ltwd| ltwd.diff_mine)
                            .unwrap_or("—".into())
                    ));
                    let diff = st
                        .best_category
                        .clone()
                        .map(|ltwd| ltwd.diff_last)
                        .unwrap_or("—".into());

                    ui.label(format!("Difference to last: {}", diff.to_string()));
                });
                columns[1].vertical(|ui| {
                    ui.heading(format!("Best time for {}", st.car_model.clone()));
                    ui.label(format!(
                        "Driver: {}",
                        st.best_car
                            .clone()
                            .map(|ltwd| ltwd.driver)
                            .unwrap_or("—".into())
                    ));
                    ui.label(format!(
                        "Time: {}",
                        st.best_car
                            .clone()
                            .map(|ltwd| ltwd.time)
                            .unwrap_or("—".into())
                    ));
                    ui.label(format!(
                        "Difference to mine: {}",
                        st.best_car
                            .clone()
                            .map(|ltwd| ltwd.diff_mine)
                            .unwrap_or("—".into())
                    ));
                    ui.label(format!(
                        "Difference to last: {}",
                        st.best_car
                            .clone()
                            .map(|ltwd| ltwd.diff_last)
                            .unwrap_or("—".into())
                    ));
                });
            });
            ui.separator();
            ui.label(format!(
                "Session Best Lap: {}",
                st.session_best.clone().unwrap_or("—".into())
            ));
            ui.label(format!(
                "Last Lap: {}",
                st.last_lap.clone().unwrap_or("—".into())
            ));
            ui.label(format!("Laps Run: {}", st.laps_run));
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for line in &st.info_log {
                    ui.label(line);
                }
            });
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            let st = self.state.lock().unwrap();
            ui.horizontal(|ui| {
                ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
                ui.label(format!(
                    "Last Change: {} seconds ago",
                    &st.last_update
                        .clone()
                        .map(|dt| (chrono::Local::now() - dt)
                            .as_seconds_f64()
                            .round()
                            .to_string())
                        .unwrap_or("Unknown".parse().unwrap())
                ));
            });
        });

        // Request a repaint to keep UI responsive while background updates happen.
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

async fn refresh_laps(
    repo: &Repository,
    driver: &Driver,
    track: &TrackRow,
    car: &CarRow,
) -> BestLaps {
    let car_records = repo
        .best_laps_for_car(track.id, car.id)
        .await
        .unwrap_or_default();

    let category_records = repo
        .best_laps_for_category(track.id, &car.category.to_string())
        .await
        .unwrap_or_default();

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
    result.clone()
}

fn main() -> anyhow::Result<()> {
    // Build tokio runtime for background async work
    let rt = Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?,
    );

    // Shared state
    let state = Arc::new(Mutex::new(UiState {
        status: "Starting…".into(),
        driver_name: "Unknown".into(),
        track_name: "—".into(),
        car_model: "—".into(),
        category_name: "_".into(),
        laps_run: 0,
        last_lap: None,
        session_best: None,
        best_car: None,
        best_category: None,
        info_log: vec![],
        last_update: None,
    }));

    // Kick off background task
    {
        let state_bg = state.clone();
        rt.spawn(async move {
            dotenv().ok();
            let repo = Repository::connect(&env::var("DATABASE_URL").unwrap(), 5)
                .await
                .unwrap();

            let driver_name = env::var("DRIVER_NAME").expect("DRIVER_NAME is not set");
            let driver_row = repo.upsert_driver(&driver_name).await.unwrap();

            let discord_url = env::var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOK is not set");
            let notifier =
                DiscordNotifier::new(discord_url, format!("{}'s ACC Bot", driver_row.name))
                    .unwrap();
            {
                let mut s = state_bg.lock().unwrap();
                s.status = "Connecting…".into();
                s.driver_name = driver_row.clone().name;
                s.last_update = Some(chrono::Local::now());
            }

            let mut telemetry = AccFactory::new()
                .connect(Duration::from_secs(1))
                .await
                .unwrap();

            loop {
                while !telemetry.connected().await {
                    {
                        let mut s = state_bg.lock().unwrap();
                        s.status = "Waiting for session…".into();
                        // Reset state to default values
                        s.car_model = "".into();
                        s.category_name = "".into();
                        s.track_name = "".into();
                        s.session_best = None;
                        s.last_lap = None;
                        s.laps_run = 0;
                        s.best_car = None;
                        s.best_category = None;
                        s.last_update = Some(chrono::Local::now());
                    }
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
                {
                    let mut s = state_bg.lock().unwrap();
                    s.status = "Connected".into();
                    s.last_update = Some(chrono::Local::now());
                }

                while telemetry.connected().await
                    && telemetry
                        .next_state()
                        .await
                        .unwrap()
                        .session_type
                        .ne(&SessionType::Hotlap)
                {
                    {
                        let mut s = state_bg.lock().unwrap();
                        s.status = "Only Hotlap is supported".into();
                        s.last_update = Some(chrono::Local::now());
                    }
                }
                let telemetry_static_info = telemetry.static_info();
                let track_name = telemetry_static_info.track_name;
                let car_model = telemetry_static_info.car_model;
                let track: TrackName = track_name.parse().expect("Unknown track");
                let track_row = repo.upsert_track(&track_name).await.unwrap();

                let car = Car::from_str(car_model.as_str()).expect("Unknown car model");
                let car_row = repo
                    .upsert_car(&car_model, &car.category.to_string())
                    .await
                    .unwrap();
                {
                    let mut s = state_bg.lock().unwrap();
                    s.track_name = track.to_string();
                    s.category_name = car.category.to_string();
                    s.car_model = car.name.to_string();
                    s.last_update = Some(chrono::Local::now());
                    s.laps_run = 0;
                }

                let mut lap_number = 0;
                let mut bests = refresh_laps(&repo, &driver_row, &track_row, &car_row).await;
                {
                    let mut s = state_bg.lock().unwrap();
                    s.status = "Waiting for lap".into();
                    s.best_category = bests.category.overall.clone().map(|best| LapTimeWithDiff {
                        time: format_lap_time(Option::from(best.clone())),
                        diff_mine: diff_lap_time(
                            bests.category.mine.clone().map(|mine| mine.lap_time_ms()),
                            best.clone(),
                        ),
                        diff_last: diff_lap_time(None, best.clone()),
                        driver: best.clone().driver_name,
                    });
                    s.best_car = bests.car.overall.clone().map(|best| LapTimeWithDiff {
                        time: format_lap_time(Option::from(best.clone())),
                        diff_mine: diff_lap_time(
                            bests.car.mine.clone().map(|mine| mine.lap_time_ms()),
                            best.clone(),
                        ),
                        diff_last: diff_lap_time(None, best.clone()),
                        driver: best.clone().driver_name,
                    });
                    s.last_update = Some(chrono::Local::now());
                }
                let mut session_best = None;
                let mut session_best_ms = None;

                while let Some(sim_state) = telemetry.next_state().await {
                    if sim_state.status.ne(&Status::Live) && sim_state.status.ne(&Status::Pause) {
                        {
                            let mut s = state_bg.lock().unwrap();
                            s.status = "Session ended".into();
                            s.car_model = "".into();
                            s.track_name = "".into();
                            s.session_best = None;
                            s.last_lap = None;
                            s.laps_run = 0;
                            s.best_car = None;
                            s.best_category = None;
                            s.last_update = Some(chrono::Local::now());
                        }
                        println!(
                            "Session {} {} ended",
                            car.name.to_string(),
                            track.to_string()
                        );
                        continue;
                    }

                    let mut refresh = false;
                    let mut latest_log = None;
                    println!("Session best lap: {:?}", sim_state.lap_timing.clone());

                    if sim_state.completed_laps.ne(&lap_number) {
                        lap_number = sim_state.completed_laps;
                        refresh = true;
                        latest_log = Some(format!(
                            "Finished lap {} with time {}",
                            lap_number,
                            sim_state.lap_timing.last_text.clone().unwrap_or("-".into())
                        ));
                        println!("{}", latest_log.clone().unwrap());

                        if sim_state.lap_timing.best_ms.is_some() {
                            session_best = sim_state.lap_timing.best_text.clone();
                            session_best_ms = sim_state.lap_timing.best_ms.clone();
                            let current_best_time = sim_state.lap_timing.best_ms.unwrap();
                            if bests.car.mine.clone().is_none()
                                || (bests
                                    .car
                                    .mine
                                    .clone()
                                    .is_some_and(|t| current_best_time < t.lap_time_ms))
                            {
                                let new_best_time = BestLapData {
                                    driver_id: driver_row.id,
                                    track_id: track_row.id,
                                    created_at: chrono::Utc::now(),
                                    lap_time_ms: current_best_time,
                                    car_id: car_row.id,
                                };

                                repo.upsert_best_lap(&new_best_time).await.unwrap();

                                let faster_by = bests
                                    .car
                                    .mine
                                    .clone()
                                    .map(|t| {
                                        format!(
                                            " (-{}ms)",
                                            pad_lap_segment(
                                                (t.lap_time_ms - new_best_time.lap_time_ms) as u64,
                                                3
                                            )
                                            .to_string()
                                        )
                                    })
                                    .unwrap_or("".to_string());

                                let fastest_for_category = bests
                                    .category
                                    .overall
                                    .clone()
                                    .map(|t| t.lap_time_ms > new_best_time.lap_time_ms)
                                    .unwrap_or(false);
                                let fastest_for_car = bests
                                    .car
                                    .overall
                                    .clone()
                                    .map(|t| t.lap_time_ms > new_best_time.lap_time_ms)
                                    .unwrap_or(false);
                                let my_fastest_for_category = bests
                                    .category
                                    .mine
                                    .clone()
                                    .map(|t| t.lap_time_ms > new_best_time.lap_time_ms)
                                    .unwrap_or(false);

                                let message_prefix = if fastest_for_category {
                                    format!("{} fastest", car.category).to_string()
                                } else if fastest_for_car {
                                    "Car fastest".to_string()
                                } else if my_fastest_for_category {
                                    format!("{} PB", car.category).to_string()
                                } else {
                                    "Car PB".to_string()
                                };

                                notifier
                                    .send(format!(
                                        "{message_prefix} {}{faster_by} in {} on {}",
                                        format_lap_time(Some(new_best_time.clone())),
                                        car.name,
                                        track
                                    ))
                                    .await
                                    .unwrap_or_default();
                                refresh = true;
                            }
                        }
                    }

                    if refresh {
                        bests = refresh_laps(&repo, &driver_row, &track_row, &car_row).await;
                    }

                    {
                        let mut s = state_bg.lock().unwrap();
                        s.status =
                            format!("Processing session in state: {:?}", sim_state.status).into();
                        s.laps_run = lap_number.into();
                        s.last_lap = sim_state.lap_timing.last_text.clone();
                        s.session_best = session_best.clone();
                        s.last_update = Some(chrono::Local::now());
                        s.best_category =
                            bests.category.overall.clone().map(|best| LapTimeWithDiff {
                                time: format_lap_time(Option::from(best.clone())),
                                diff_mine: diff_lap_time(
                                    bests.category.mine.clone().map(|mine| mine.lap_time_ms()),
                                    best.clone(),
                                ),
                                diff_last: diff_lap_time(session_best_ms, best.clone()),
                                driver: best.clone().driver_name,
                            });
                        s.best_car = bests.car.overall.clone().map(|best| LapTimeWithDiff {
                            time: format_lap_time(Option::from(best.clone())),
                            diff_mine: diff_lap_time(
                                bests.car.mine.clone().map(|mine| mine.lap_time_ms()),
                                best.clone(),
                            ),
                            diff_last: diff_lap_time(session_best_ms, best.clone()),
                            driver: best.clone().driver_name,
                        });
                    }
                }
            }
        });
    }

    let options = NativeOptions::default();
    let app = MyApp::new(rt, state);
    Ok(eframe::run_native(
        "ACC lap-time leaderboard recorder",
        options,
        Box::new(|_| Ok(Box::new(app))),
    )
    .unwrap())
}
