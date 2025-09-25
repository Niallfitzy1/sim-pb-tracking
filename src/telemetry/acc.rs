use crate::telemetry::{LapTiming, SimState, StaticInfo, TelemetryClient, TelemetryFactory};
use anyhow::Result;
use simetry::assetto_corsa_competizione::Client;
use std::time::Duration;

pub struct AccFactory;

impl AccFactory {
    pub fn new() -> Self {
        Self
    }
}

impl TelemetryFactory for AccFactory {
    async fn connect(&self, poll_interval: Duration) -> Result<Box<dyn TelemetryClient>> {
        Ok(Box::new(AccClient {
            inner: Client::connect(poll_interval).await,
        }))
    }
}

pub struct AccClient {
    inner: Client,
}

#[async_trait::async_trait]
impl TelemetryClient for AccClient {
    async fn connected(&mut self) -> bool {
        self.inner.next_sim_state().await.is_some()
    }

    fn static_info(&self) -> StaticInfo {
        let s = self.inner.static_data();
        StaticInfo {
            track_name: s.track.clone(),
            car_model: s.car_model.clone(),
        }
    }

    async fn next_state(&mut self) -> Option<SimState> {
        match self.inner.next_sim_state().await {
            Some(s) => {
                let g = s.graphics;
                let last_ms = if g.lap_timing.last.millis < i32::MAX {
                    Some(g.lap_timing.last.millis as i64)
                } else {
                    None
                };
                let best_ms = if g.lap_timing.best.millis < i32::MAX {
                    Some(g.lap_timing.best.millis as i64)
                } else {
                    None
                };
                let best_text = if best_ms.is_some() {
                    Some(g.lap_timing.best.text.clone())
                } else {
                    None
                };
                let last_text = if last_ms.is_some() {
                    Some(g.lap_timing.last.text.clone())
                } else {
                    None
                };
                Some(SimState {
                    status: g.status.clone(),
                    session_type: g.session.clone(),
                    completed_laps: g.completed_laps,
                    lap_timing: LapTiming {
                        last_ms,
                        best_ms,
                        best_text,
                        last_text,
                    },
                })
            }
            None => None,
        }
    }
}
