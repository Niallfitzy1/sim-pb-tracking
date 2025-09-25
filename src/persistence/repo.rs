use crate::persistence::models::*;
use anyhow::Result;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct Repository {
    pub pool: Pool<Postgres>,
}

impl Repository {
    pub async fn connect(database_url: &str, max_connections: u32) -> Result<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn upsert_driver(&self, name: &str) -> Result<Driver> {
        let d = sqlx::query_as!(
            Driver,
            r#"INSERT INTO driver (name)
               VALUES ($1)
               ON CONFLICT (name) DO UPDATE SET name=$1
               RETURNING id, name"#,
            name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(d)
    }

    pub async fn upsert_track(&self, name: &str) -> Result<TrackRow> {
        let t = sqlx::query_as!(
            TrackRow,
            r#"INSERT INTO track (name)
               VALUES ($1)
               ON CONFLICT (name) DO UPDATE SET name=$1
               RETURNING id, name"#,
            name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(t)
    }

    pub async fn upsert_car(&self, name: &str, category: &str) -> Result<CarRow> {
        let c = sqlx::query_as!(
            CarRow,
            r#"INSERT INTO car (name, category)
               VALUES ($1, $2)
               ON CONFLICT (name) DO UPDATE SET name=$1
               RETURNING id, name, category"#,
            name,
            category
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(c)
    }

    pub async fn upsert_best_lap(&self, data: &BestLapData) -> Result<BestLap> {
        let b = sqlx::query_as!(
            BestLap,
            r#"INSERT INTO best_lap (driver_id, track_id, car_id, created_at, lap_time_ms)
               VALUES ($1, $2, $3, $4, $5)
               ON CONFLICT (driver_id, track_id, car_id)
               DO UPDATE SET lap_time_ms=$5
               RETURNING id, driver_id, track_id, car_id, created_at, lap_time_ms"#,
            data.driver_id,
            data.track_id,
            data.car_id,
            data.created_at,
            data.lap_time_ms
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(b)
    }

    pub async fn best_laps_for_car(
        &self,
        track_id: i64,
        car_id: i64,
    ) -> Result<Vec<BestLapWithDriver>> {
        let rows = sqlx::query_as!(
            BestLapWithDriver,
            r#"SELECT bl.id,
                      bl.track_id,
                      bl.driver_id,
                      bl.lap_time_ms,
                      bl.created_at,
                      bl.car_id,
                      d.name       as driver_name,
                      c.name       as car_name,
                      c.category   as car_category
               FROM best_lap bl
               INNER JOIN driver d ON d.id = bl.driver_id
               INNER JOIN car c ON c.id = bl.car_id
               WHERE bl.track_id = $1 AND c.id = $2
               ORDER BY bl.lap_time_ms ASC"#,
            track_id,
            car_id as i32
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn best_laps_for_category(
        &self,
        track_id: i64,
        category: &str,
    ) -> Result<Vec<BestLapWithDriver>> {
        let rows = sqlx::query_as!(
            BestLapWithDriver,
            r#"SELECT bl.id,
                      bl.track_id,
                      bl.driver_id,
                      bl.lap_time_ms,
                      bl.created_at,
                      bl.car_id,
                      d.name       as driver_name,
                      c.name       as car_name,
                      c.category   as car_category
               FROM best_lap bl
               INNER JOIN driver d ON d.id = bl.driver_id
               INNER JOIN car c ON c.id = bl.car_id
               WHERE bl.track_id = $1 AND c.category = $2
               ORDER BY bl.lap_time_ms ASC"#,
            track_id,
            category
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}
