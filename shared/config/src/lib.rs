mod tests;

use gloo::storage::{LocalStorage, Storage};
use invidious::{AudioQuality, VideoQuality};
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use utils::save_to_browser_storage;

use crate::RememberPosition::VideosOnly;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct NetworkConfig {
	pub server: String,
	pub custom_servers: Option<Vec<String>>,
	pub auto_fetch_subs: bool,
	pub fetch_rss: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct UiConfig {
	pub theme: String,
	pub homepage: HomepageCategory,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PlayerConfig {
	pub auto_play: bool,
	pub fast_forward_interval: u8,
	pub default_video_quality: VideoQuality,
	pub default_audio_quality: AudioQuality,
	pub remember_position: RememberPosition,
	pub volume: f64,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyConfig {
	pub keep_history: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct RegionConfig {
	pub locale: RustyTubeLocale,
	pub trending_region: isocountry::CountryCode,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Config {
	pub network: NetworkConfig,
	pub ui: UiConfig,
	pub player: PlayerConfig,
	pub privacy: PrivacyConfig,
	pub region: RegionConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RememberPosition {
	Always,
	VideosOnly,
	Never,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HomepageCategory {
	Trending,
	Popular,
	Subscriptions,
}

impl Default for NetworkConfig {
	fn default() -> Self {
		let server = String::from("https://invidious.fdn.fr");
		let custom_servers = None;
		let auto_fetch_subs = true;
		let fetch_rss = false;

		Self { server, custom_servers, auto_fetch_subs, fetch_rss }
	}
}

impl Default for UiConfig {
	fn default() -> Self {
		let theme = String::from("rustytube");
		let homepage = HomepageCategory::Subscriptions;

		Self { theme, homepage }
	}
}

impl Default for PlayerConfig {
	fn default() -> Self {
		let auto_play = true;
		let fast_forward_interval = 10u8;
		let default_video_quality = VideoQuality::_1080p;
		let default_audio_quality = AudioQuality::Medium;
		let remember_position = VideosOnly;
		let volume = 0.5f64;

		Self {
			auto_play,
			volume,
			fast_forward_interval,
			default_video_quality,
			default_audio_quality,
			remember_position,
		}
	}
}

impl Default for PrivacyConfig {
	fn default() -> Self {
		let keep_history = true;

		Self { keep_history }
	}
}

impl Default for RegionConfig {
	fn default() -> Self {
		let locale = RustyTubeLocale::EN_US;
		let trending_region = isocountry::CountryCode::IRL;

		Self { locale, trending_region }
	}
}

impl Default for Config {
	fn default() -> Self {
		let network = NetworkConfig::default();
		let ui = UiConfig::default();
		let player = PlayerConfig::default();
		let privacy = PrivacyConfig::default();
		let region = RegionConfig::default();

		Self { network, ui, player, privacy, region }
	}
}

pub const CONFIG_KEY: &'static str = "RUSTYTUBE_CONFIG";

impl Config {
	pub fn save(&self) -> Result<(), RustyTubeError> {
		save_to_browser_storage(CONFIG_KEY, &self.to_toml_string()?)?;
		Ok(())
	}

	pub fn load() -> Result<Self, RustyTubeError> {
		let config_str = LocalStorage::get::<String>(CONFIG_KEY)?;
		Self::from_toml_string(&config_str)
	}

	pub fn to_toml_string(&self) -> Result<String, RustyTubeError> {
		Ok(toml::to_string(&self)?)
	}

	pub fn from_toml_string(toml_str: &str) -> Result<Self, RustyTubeError> {
		Ok(toml::from_str(toml_str)?)
	}
}
