use std::path::Path;

use gloo::storage::{LocalStorage, Storage};
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use tauri_sys::fs::{read_text_file, write_text_file, BaseDirectory};

pub async fn load_resource<T>(key: impl AsRef<str>) -> Result<T, RustyTubeError>
where
	T: for<'a> Deserialize<'a>,
{
	let data = if cfg!(feature = "tauri") {
		let text_data = read_text_file(
			Path::new(&format!("RustyTube/{}.ron", key.as_ref())),
			BaseDirectory::AppData,
		)
		.await?;
		ron::from_str(&text_data)?
	} else {
		LocalStorage::get::<T>(key)?
	};
	Ok(data)
}

pub async fn save_resource<T>(
	key: impl AsRef<str>,
	data: T,
) -> Result<(), RustyTubeError>
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
		.await?;
	}
	Ok(())
}
