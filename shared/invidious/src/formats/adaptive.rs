use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::formats::{
	items::{ColorInfo, Container, Resolution, VideoQuality},
	AudioFormat, AudioQuality, VideoFormat,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveFormat {
	pub init: String,
	pub index: String,
	pub bitrate: String,
	pub url: String,
	pub itag: String,
	pub r#type: String,
	pub clen: String,
	pub lmt: String,
	pub projection_type: Option<String>,
	pub fps: Option<u8>,
	pub audio_quality: Option<AudioQuality>,
	pub audio_sample_rate: Option<u32>,
	pub audio_channels: Option<u32>,
	pub container: Option<Container>,
	pub encoding: Option<String>,
	pub resolution: Option<Resolution>,
	pub quality_label: Option<VideoQuality>,
	pub color_info: Option<ColorInfo>,
}

impl TryFrom<AdaptiveFormat> for AudioFormat {
	type Error = RustyTubeError;

	fn try_from(adaptive_format: AdaptiveFormat) -> Result<Self, Self::Error> {
		let audio_quality = adaptive_format
			.audio_quality
			.ok_or(RustyTubeError::NoAudioFormat)?;
		let audio_sample_rate = adaptive_format
			.audio_sample_rate
			.ok_or(RustyTubeError::NoAudioFormat)?;
		let audio_channels = adaptive_format
			.audio_channels
			.ok_or(RustyTubeError::NoAudioFormat)?;

		Ok(Self {
			init: adaptive_format.init,
			index: adaptive_format.index,
			bitrate: adaptive_format.bitrate,
			url: adaptive_format.url,
			itag: adaptive_format.itag,
			r#type: adaptive_format.r#type,
			clen: adaptive_format.clen,
			lmt: adaptive_format.lmt,
			projection_type: adaptive_format.projection_type,
			container: adaptive_format.container,
			encoding: adaptive_format.encoding,
			audio_quality,
			audio_sample_rate,
			audio_channels,
		})
	}
}

impl TryFrom<AdaptiveFormat> for VideoFormat {
	type Error = RustyTubeError;

	fn try_from(adaptive_format: AdaptiveFormat) -> Result<Self, Self::Error> {
		let resolution = adaptive_format
			.resolution
			.ok_or(RustyTubeError::NoAdaptiveFormat)?;
		let quality_label = adaptive_format
			.quality_label
			.ok_or(RustyTubeError::NoAdaptiveFormat)?;
		let fps =
			adaptive_format.fps.ok_or(RustyTubeError::NoAdaptiveFormat)?;

		Ok(Self {
			init: adaptive_format.init,
			index: adaptive_format.index,
			bitrate: adaptive_format.bitrate,
			url: adaptive_format.url,
			itag: adaptive_format.itag,
			r#type: adaptive_format.r#type,
			clen: adaptive_format.clen,
			lmt: adaptive_format.lmt,
			projection_type: adaptive_format.projection_type,
			container: adaptive_format.container,
			encoding: adaptive_format.encoding,
			fps,
			resolution,
			quality_label,
			color_info: adaptive_format.color_info,
		})
	}
}
