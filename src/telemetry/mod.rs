pub mod acc;

use std::time::Duration;

#[derive(Clone, Debug)]
pub struct StaticInfo {
    pub track_name: String,
    pub car_model: String,
}

#[derive(Clone, Debug)]
pub struct LapTiming {
    pub last_ms: Option<i64>,
    pub best_ms: Option<i64>,
    pub best_text: Option<String>,
    pub last_text: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SimState {
    pub completed_laps: i32,
    pub lap_timing: LapTiming,
}

#[async_trait::async_trait]
pub trait TelemetryClient: Send {
    fn static_info(&self) -> StaticInfo;
    async fn connected(&mut self) -> bool;
    async fn next_state(&mut self) -> Option<SimState>;
}

pub trait TelemetryFactory: Send + Sync {
    async fn connect(&self, poll_interval: Duration) -> anyhow::Result<Box<dyn TelemetryClient>>;
}
