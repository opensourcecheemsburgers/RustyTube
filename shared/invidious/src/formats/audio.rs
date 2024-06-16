use serde::{Deserialize, Serialize};

use crate::formats::items::{AudioQuality, Container};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFormat {
	pub init: String,
	pub index: String,
	pub bitrate: String,
	pub url: String,
	pub itag: String,
	pub r#type: String,
	pub clen: String,
	pub lmt: String,
	pub projection_type: Option<String>,
	pub container: Option<Container>,
	pub encoding: Option<String>,
	pub audio_quality: AudioQuality,
	pub audio_sample_rate: u32,
	pub audio_channels: u32,
}
