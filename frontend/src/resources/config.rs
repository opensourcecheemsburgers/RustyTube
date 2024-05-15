use std::path::Path;

use config::Config;
use leptos::*;
use rustytube_error::RustyTubeError;
use tauri_sys::fs::{read_text_file, BaseDirectory};

static CONFIG_KEY: &'static str = "RUSTYTUBE_CONFIG";

#[derive(Clone, Copy)]
pub struct ConfigResource {
	pub resource: Resource<(), Result<Config, RustyTubeError>>,
}

impl ConfigResource {
	pub fn initialise() -> Self {
		let resource = create_local_resource_with_initial_value(
			move || (),
			move |()| fetch_config(),
			Some(Config::load()),
		);

		ConfigResource { resource }
	}
}

async fn fetch_config() -> Result<Config, RustyTubeError> {
	match cfg!(feature = "tauri") {
		true => {
			let text_data =
				read_text_file(&Path::new("RustyTube/config.toml"), BaseDirectory::AppData).await?;
			Config::from_toml_string(&text_data)
		}
		false => Config::load(),
	}
}
