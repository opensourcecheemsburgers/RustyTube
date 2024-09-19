mod utils {
	use std::{collections::HashMap, ops::Div, time::Duration};

	use chrono::DateTime;
	use gloo::{
		storage::{LocalStorage, Storage},
		utils::document,
	};
	use humantime::format_duration;
	use rustytube_error::RustyTubeError;
	use serde_json::Value;
	use wasm_bindgen::JsCast;

	/// # Errors
	///
	/// - Element with provided id cannot be found.
	/// - Element cannot be dynamically converted into the provided type.
	pub fn get_element_by_id<T>(id: &str) -> Result<T, RustyTubeError>
	where
		T: JsCast,
	{
		let element = document()
			.get_element_by_id(id)
			.ok_or(RustyTubeError::ElementNotFound)?
			.dyn_into::<T>()
			.ok()
			.ok_or(RustyTubeError::DynInto)?;
		Ok(element)
	}

	/// # Errors
	///
	/// - `LocalStorage` save failure.
	pub fn save_to_browser_storage(
		key: &str,
		value: &str,
	) -> Result<(), RustyTubeError> {
		LocalStorage::set(key, value)?;
		Ok(())
	}

	/// # Errors
	///
	/// - `LocalStorage` load failure.
	pub fn load_all_from_browser_storage(
	) -> Result<HashMap<String, Value>, RustyTubeError> {
		let storage = LocalStorage::get_all()?;
		Ok(storage)
	}

	pub fn unix_to_hours_secs_mins(secs: f64) -> String {
		let duration = Duration::from_secs_f64(secs);
		let seconds = duration.as_secs() % 60;
		let minutes = (duration.as_secs() / 60) % 60;
		let hours = (duration.as_secs() / 60) / 60;

		if hours > 0 {
			format!("{hours:0>1}:{minutes:0>2}:{seconds:0>2}")
		} else {
			format!("{minutes:0>1}:{seconds:0>2}")
		}
	}

	/// # Errors
	///
	/// - No `Window`.
	/// - No `Performance`.
	pub fn get_current_time_ms() -> Result<f64, RustyTubeError> {
		let window = web_sys::window().ok_or(RustyTubeError::NoWindow)?;
		let performance =
			window.performance().ok_or(RustyTubeError::NoPerformance)?;
		let current_time = performance.now();
		Ok(current_time)
	}

	/// # Errors
	///
	/// - No `Window`.
	/// - No `Performance`.
	pub fn get_current_time() -> Result<f64, RustyTubeError> {
		let current_time = get_current_time_ms()?.div(1000f64);
		Ok(current_time)
	}

	/// # Errors
	///
	/// - No `Window`.
	/// - No `Performance`.
	#[allow(clippy::cast_possible_truncation)]
	pub fn get_current_time_rfc() -> Result<String, RustyTubeError> {
		let current_time_ms = get_current_time_ms()? as i64;
		let current_time_rfc = DateTime::from_timestamp_millis(current_time_ms)
			.ok_or(RustyTubeError::DateTime)?
			.to_rfc3339();
		Ok(current_time_rfc)
	}

	/// # Errors
	///
	/// - No `Window`.
	/// - No `Performance`.
	#[allow(clippy::cast_precision_loss)]
	pub fn get_time_since(time: u64) -> Result<String, RustyTubeError> {
		let diff = get_current_time()? - (time as f64);
		let diff_duration = Duration::from_secs_f64(diff);
		let formatted_diff = format_duration(diff_duration).to_string();
		Ok(formatted_diff)
	}

	/// # Errors
	///
	/// - No `Window`.
	/// - No `Performance`.
	#[allow(clippy::cast_precision_loss)]
	pub fn get_time_until(time: u64) -> Result<String, RustyTubeError> {
		let diff = (time as f64) - get_current_time()?;
		let diff_duration = Duration::from_secs_f64(diff);
		let formatted_diff = format_duration(diff_duration).to_string();
		Ok(formatted_diff)
	}

	/// # Errors
	///
	/// - No `Window`.
	/// - No `Performance`.
	/// - RFC3339 Parse Error.
	#[allow(clippy::cast_precision_loss)]
	pub fn get_published_time(rfc: &str) -> Result<String, RustyTubeError> {
		let video_time = DateTime::parse_from_rfc3339(rfc)?.timestamp() as f64;
		let diff_duration = Duration::from_secs_f64(video_time);
		Ok(format_duration(diff_duration).to_string())
	}

	/// # Errors
	///
	/// - No `Window`.
	/// - No `Performance`.
	/// - RFC3339 Parse Error.
	#[allow(clippy::cast_sign_loss)]
	pub fn get_published_time_ms(rfc: &str) -> Result<u64, RustyTubeError> {
		let video_time = DateTime::parse_from_rfc3339(rfc)?.timestamp() as u64;
		Ok(video_time)
	}
}

pub use utils::*;
