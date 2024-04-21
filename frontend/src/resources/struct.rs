use std::path::Path;

use gloo::storage::{LocalStorage, Storage};
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use tauri_sys::fs::*;

pub async fn load_resource<T>(key: impl AsRef<str>) -> Result<T, RustyTubeError>
where
	T: for<'a> Deserialize<'a>,
{
	let data = match cfg!(feature = "tauri") {
		true => {
			let text_data = read_text_file(
				&Path::new(&format!("RustyTube/{}.ron", key.as_ref())),
				BaseDirectory::AppData,
			)
			.await?;
			ron::from_str(&text_data)?
		}
		false => LocalStorage::get::<T>(key)?,
	};
	Ok(data)
}

pub async fn save_resource<T>(key: impl AsRef<str>, data: T) -> Result<(), RustyTubeError>
where
	T: Serialize,
{
	gloo::storage::LocalStorage::set(key.as_ref(), &data)?;
	if cfg!(feature = "tauri") {
		write_text_file(
			Path::new(&format!("RustyTube/{}.ron", key.as_ref())),
			&ron::to_string(&data)?,
			BaseDirectory::AppData,
		)
		.await?
	}
	Ok(())
}

pub fn initial_value<T>(key: impl AsRef<str>) -> Option<T>
where
	T: for<'de> Deserialize<'de>,
{
	LocalStorage::get::<T>(key.as_ref()).ok()
}
