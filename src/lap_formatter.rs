use crate::persistence::LapTime;
use std::time::Duration;

pub fn pad_lap_segment(segment: u64, length: usize) -> String {
    let input = segment.to_string();
    if input.len() >= length {
        input.to_string()
    } else {
        let mut padded = String::with_capacity(length);
        for _ in 0..(length - input.len()) {
            padded.push('0');
        }
        padded.push_str(&*input);
        padded
    }
}

pub fn format_lap_time<T: LapTime>(lap_time: Option<T>) -> String {
    lap_time
        .map(|t| {
            let lap_duration = Duration::from_millis(t.lap_time_ms() as u64);
            duration_as_string(lap_duration)
        })
        .unwrap_or("None".to_string())
}

pub fn duration_as_string(lap_duration: Duration) -> String {
    format!(
        "{}:{}:{}",
        lap_duration.as_secs() / 60,
        pad_lap_segment(lap_duration.as_secs() % 60, 2),
        pad_lap_segment(lap_duration.subsec_millis() as u64, 3),
    )
}

pub fn diff_lap_time<T: LapTime>(previous_lap_time: Option<i64>, new_lap_time: T) -> String {
    if previous_lap_time.is_none() {
        return "∞ (No previous lap time)".to_string();
    }
    let new_duration = new_lap_time.lap_time_ms();
    let diff = new_duration - previous_lap_time.unwrap();
    if diff.eq(&0) {
        return "—".to_string();
    }

    format!("{}", diff)
}
