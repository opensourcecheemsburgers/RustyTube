mod utils {
    use chrono::{DateTime, Utc};
    use gloo::storage::{LocalStorage, Storage};
    use gloo::utils::document;
    use humantime::format_duration;
    use rustytube_error::RustyTubeError;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::time::Duration;
    use wasm_bindgen::JsCast;

    pub fn get_element_by_id<T>(id: &str) -> Result<T, RustyTubeError>
    where
        T: JsCast,
    {
        let element = document()
            .get_element_by_id(id)
            .ok_or(RustyTubeError::element_not_found(id))?
            .dyn_into::<T>()
            .ok()
            .ok_or(RustyTubeError::dyn_into_fail(id))?;
        Ok(element)
    }

    pub fn save_to_browser_storage(key: &str, value: &str) -> Result<(), RustyTubeError> {
        LocalStorage::set(key, value)?;
        Ok(())
    }

    pub fn load_all_from_browser_storage() -> Result<HashMap<String, Value>, RustyTubeError> {
        let storage = LocalStorage::get_all()?;
        Ok(storage)
    }

    pub fn unix_to_hours_secs_mins(secs: f64) -> String {
        let duration = Duration::from_secs_f64(secs);
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        let hours = (duration.as_secs() / 60) / 60;

        match hours > 0 {
            true => format!("{:0>1}:{:0>2}:{:0>2}", hours, minutes, seconds),
            false => format!("{:0>1}:{:0>2}", minutes, seconds),
        }
    }

    pub fn get_time_since(time: u64) -> String {
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let current_perf = performance.now();

        let current_time = (current_perf as u64) / 1_000;
        let diff = current_time - time;

        let diff_duration = Duration::from_millis(diff);
        format_duration(diff_duration).to_string()
    }

    pub fn get_time_until(time: u64) -> String {
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let current_perf = performance.now();

        let current_time = (current_perf as u64) / 1_000;
        let diff = time - current_time;

        let diff_duration = Duration::from_millis(diff);
        format_duration(diff_duration).to_string()
    }

    pub fn get_current_time() -> u64 {
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let current_perf = performance.now();

        (current_perf as u64) / 1_000
    }

    pub fn get_current_time_rfc() -> String {
        Utc::now().to_rfc3339()
    }

    pub fn get_published_time_ms(rfc: &str) -> Result<u64, RustyTubeError> {
        Ok(DateTime::parse_from_rfc3339(&rfc)?.timestamp() as u64)
    }

    pub fn get_published_time(rfc: &str) -> Result<String, RustyTubeError> {
        let video_time = DateTime::parse_from_rfc3339(&rfc)?.timestamp_millis() as u64;
        let current_time = get_current_time();
        let diff = current_time - video_time;
        let elapsed = Duration::from_millis(diff);
        Ok(format_duration(elapsed).to_string())
    }
}

pub use utils::*;

