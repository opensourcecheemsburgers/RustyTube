mod tests;

use serde::{Deserialize, Serialize};
use figment::{Figment, providers::{Format, Serialized, Toml}};
use gloo::storage::{LocalStorage, Storage};
use invidious::hidden::Quality;
use invidious::hidden::Quality::_1080p;
use rustytube_error::RustyTubeError;
use utils::save_to_browser_storage;
use crate::RememberPosition::VideosOnly;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct NetworkConfig {
    pub server: String,
    pub custom_servers: Option<Vec<String>>,
    pub auto_fetch_subs: bool,
    pub fetch_rss: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct UiConfig {
    pub theme: String,
    pub font_scale: u8,
    pub ui_scale: u8,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct PlayerConfig {
    pub auto_play: bool,
    pub fast_forward_interval: u8,
    pub default_quality: Quality,
    pub remember_position: RememberPosition,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct PrivacyConfig {
    pub keep_history: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    pub network: NetworkConfig,
    pub ui: UiConfig,
    pub player: PlayerConfig,
    pub privacy: PrivacyConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RememberPosition {
    Always,
    VideosOnly,
    Never
}

impl Default for NetworkConfig {
    fn default() -> Self {
        let server = String::from("https://iv.nboeck.de");
        let custom_servers = None;
        let auto_fetch_subs = true;
        let fetch_rss = false;

        Self { server, custom_servers, auto_fetch_subs, fetch_rss }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        let theme = String::from("dracula");
        let font_scale = 100u8;
        let ui_scale = 100u8;

        Self { theme, font_scale, ui_scale }
    }
}

impl Default for PlayerConfig {
    fn default() -> Self {
        let auto_play = true;
        let fast_forward_interval = 10u8;
        let default_quality = _1080p;
        let remember_position = VideosOnly;

        Self { auto_play, fast_forward_interval, default_quality, remember_position }
    }
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        let keep_history = true;

        Self { keep_history }
    }
}

impl Default for Config {
    fn default() -> Self {
        let network = NetworkConfig::default();
        let ui = UiConfig::default();
        let player = PlayerConfig::default();
        let privacy = PrivacyConfig::default();

        Self { network, ui, player, privacy }
    }
}

pub const CONFIG_KEY: &'static str = "RUSTYTUBE_CONFIG";

impl Config {
    pub fn save(&self) -> Result<(), RustyTubeError> {
        save_to_browser_storage(CONFIG_KEY, &self.to_toml_string()?)?;
        Ok(())
    }

    pub fn load() -> Result<Self, RustyTubeError> {
        Ok(LocalStorage::get(CONFIG_KEY)?)
    } 

    pub fn to_toml_string(&self) -> Result<String, RustyTubeError> {
        Ok(toml::to_string(&self)?)
    }

    pub fn from_toml_string(toml_str: &str) -> Result<Self, RustyTubeError> {
        Ok(toml::from_str(toml_str)?)
    }
}