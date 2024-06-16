use serde::{Deserialize, Serialize};

use crate::formats::{ColorInfo, Container, Resolution, VideoQuality};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoFormat {
	pub init: String,
	pub index: String,
	pub bitrate: String,
	pub url: String,
	pub itag: String,
	pub r#type: String,
	pub clen: String,
	pub lmt: String,
	pub projection_type: Option<String>,
	pub fps: u8,
	pub container: Option<Container>,
	pub encoding: Option<String>,
	pub resolution: Resolution,
	pub quality_label: VideoQuality,
	pub color_info: Option<ColorInfo>,
}
