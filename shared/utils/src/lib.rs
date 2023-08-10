mod utils {
    use std::{time::Duration, collections::HashMap};
    use chrono::Utc;
    use gloo::storage::{LocalStorage, Storage};
    use humantime::format_duration;
    use serde_json::Value;
    use rustytube_error::RustyTubeError;

    pub fn save_to_browser_storage(key: &str, value: &str) -> Result<(), RustyTubeError>  {
        LocalStorage::set(key, value)?;
        Ok(())
    }
    
    pub fn load_all_from_browser_storage() -> Result<HashMap<String, Value>, RustyTubeError>  {
        let storage  = LocalStorage::get_all()?;
        Ok(storage)
    }

    pub fn get_time_since(time: u64) -> String {
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window.performance().expect("performance should be available");
        let current_perf = performance.now();
        
        let current_time = (current_perf as u64) / 1_000;
        let diff = current_time - time;
        
        let diff_duration = Duration::from_millis(diff);
        format_duration(diff_duration).to_string()
    }

    pub fn get_current_time() -> u64 {
        // let window = web_sys::window().expect("should have a window in this context");
        // let performance = window.performance().expect("performance should be available");
        // let current_perf = performance.now();
        
        // let current_time = (current_perf as u64) / 1_000;
        Utc::now().timestamp_millis() as u64
    }

    pub fn get_current_time_rfc() -> String {
        // let window = web_sys::window().expect("should have a window in this context");
        // let performance = window.performance().expect("performance should be available");
        // let current_perf = performance.now();

        // let current_time = (current_perf as u64) / 1_000;
        Utc::now().to_rfc3339()
    }
}

pub use utils::*;