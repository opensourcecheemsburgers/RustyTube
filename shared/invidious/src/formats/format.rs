use super::AdaptiveFormat;
use crate::{
	formats::{audio::AudioFormat, legacy::LegacyFormat, video::VideoFormat},
	DashFormat,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Formats {
	pub video_formats: Vec<VideoFormat>,
	pub audio_formats: Vec<AudioFormat>,
	pub legacy_formats: Vec<LegacyFormat>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format {
	Dash(DashFormat),
	Legacy(LegacyFormat),
	Audio(AudioFormat),
}

impl Format {
	pub const fn is_audio_only(&self) -> bool {
		match self {
			Self::Audio(_) => true,
			Self::Legacy(_) | Self::Dash(_) => false,
		}
	}

	pub const fn is_legacy(&self) -> bool {
		match self {
			Self::Legacy(_) => true,
			Self::Audio(_) | Self::Dash(_) => false,
		}
	}

	pub fn video_url(&self) -> Option<String> {
		match self {
			Self::Dash(dash) => Some(dash.video.url.clone()),
			Self::Legacy(legacy) => Some(legacy.url.clone()),
			Self::Audio(_) => None,
		}
	}

	pub fn audio_url(&self) -> Option<String> {
		match self {
			Self::Dash(dash) => Some(dash.audio.url.clone()),
			Self::Legacy(_) => None,
			Self::Audio(audio) => Some(audio.url.clone()),
		}
	}

	pub fn audio_format(&self) -> Option<AudioFormat> {
		match self {
			Self::Dash(dash) => Some(dash.audio.clone()),
			Self::Legacy(_) => None,
			Self::Audio(audio) => Some(audio.clone()),
		}
	}

	pub fn video_format(&self) -> Option<VideoFormat> {
		match self {
			Self::Dash(dash) => Some(dash.video.clone()),
			Self::Legacy(_) | Self::Audio(_) => None,
		}
	}

	pub fn legacy_format(&self) -> Option<LegacyFormat> {
		match self {
			Self::Legacy(legacy) => Some(legacy.clone()),
			Self::Audio(_) | Self::Dash(_) => None,
		}
	}
}

impl From<(Vec<AdaptiveFormat>, Vec<LegacyFormat>)> for Formats {
	fn from(formats_tuple: (Vec<AdaptiveFormat>, Vec<LegacyFormat>)) -> Self {
		let mut video_formats: Vec<VideoFormat> = Vec::new();
		let mut audio_formats: Vec<AudioFormat> = Vec::new();

		formats_tuple.0.into_iter().for_each(|adaptive_format| {
			if let Ok(audio) = AudioFormat::try_from(adaptive_format.clone()) {
				audio_formats.push(audio);
			} else if let Ok(video) = VideoFormat::try_from(adaptive_format) {
				video_formats.push(video);
			}
		});

		let legacy_formats = formats_tuple.1;

		Self { video_formats, audio_formats, legacy_formats }
	}
}
