use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub trait LapTime {
    fn lap_time_ms(&self) -> i64;
    fn created_at(&self) -> DateTime<Utc>;
    fn driver_id(&self) -> i64;
    fn track_id(&self) -> i64;
    fn car_id(&self) -> i64;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackRow {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarRow {
    pub id: i64,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct BestLap {
    pub id: i64,
    pub driver_id: i64,
    pub track_id: i64,
    pub car_id: i64,
    pub created_at: DateTime<Utc>,
    pub lap_time_ms: i64,
}

#[derive(Debug, Clone)]
pub struct BestLapData {
    pub driver_id: i64,
    pub track_id: i64,
    pub car_id: i64,
    pub created_at: DateTime<Utc>,
    pub lap_time_ms: i64,
}

#[derive(Debug, Clone)]
pub struct BestLapWithDriver {
    pub id: i64,
    pub driver_id: i64,
    pub track_id: i64,
    pub car_id: i64,
    pub created_at: DateTime<Utc>,
    pub lap_time_ms: i64,
    pub driver_name: String,
    pub car_name: String,
    pub car_category: String,
}

#[derive(Debug, Clone)]
pub struct MyLapAndBestLap<T> {
    pub mine: Option<T>,
    pub overall: Option<T>,
}

#[derive(Debug, Clone)]
pub struct BestLaps {
    pub car: MyLapAndBestLap<BestLapWithDriver>,
    pub category: MyLapAndBestLap<BestLapWithDriver>,
}

impl LapTime for BestLapData {
    fn lap_time_ms(&self) -> i64 {
        self.lap_time_ms
    }
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    fn driver_id(&self) -> i64 {
        self.driver_id
    }
    fn track_id(&self) -> i64 {
        self.track_id
    }
    fn car_id(&self) -> i64 {
        self.car_id
    }
}

impl LapTime for BestLapWithDriver {
    fn lap_time_ms(&self) -> i64 {
        self.lap_time_ms
    }
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    fn driver_id(&self) -> i64 {
        self.driver_id
    }
    fn track_id(&self) -> i64 {
        self.track_id
    }
    fn car_id(&self) -> i64 {
        self.car_id
    }
}
