use super::AdaptiveFormat;
use crate::{
	formats::{audio::AudioFormat, legacy::LegacyFormat, video::VideoFormat},
	DashFormat,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Formats {
	pub video_formats: Vec<VideoFormat>,
	pub audio_formats: Vec<AudioFormat>,
	pub legacy_formats: Vec<LegacyFormat>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
	Dash(DashFormat),
	Legacy(LegacyFormat),
	Audio(AudioFormat),
}

impl Format {
	pub fn is_audio_only(&self) -> bool {
		match self {
			Format::Dash(_) => false,
			Format::Legacy(_) => false,
			Format::Audio(_) => true,
		}
	}

	pub fn is_legacy(&self) -> bool {
		match self {
			Format::Dash(_) => false,
			Format::Legacy(_) => true,
			Format::Audio(_) => false,
		}
	}

	pub fn video_url(&self) -> Option<String> {
		match self {
			Format::Dash(dash) => Some(dash.video.url.clone()),
			Format::Legacy(legacy) => Some(legacy.url.clone()),
			Format::Audio(_) => None,
		}
	}

	pub fn audio_url(&self) -> Option<String> {
		match self {
			Format::Dash(dash) => Some(dash.audio.url.clone()),
			Format::Legacy(_) => None,
			Format::Audio(audio) => Some(audio.url.clone()),
		}
	}

	pub fn audio_format(&self) -> Option<AudioFormat> {
		match self {
			Format::Dash(dash) => Some(dash.audio.clone()),
			Format::Legacy(_) => None,
			Format::Audio(audio) => Some(audio.clone()),
		}
	}

	pub fn video_format(&self) -> Option<VideoFormat> {
		match self {
			Format::Dash(dash) => Some(dash.video.clone()),
			Format::Legacy(_) => None,
			Format::Audio(_) => None,
		}
	}

	pub fn legacy_format(&self) -> Option<LegacyFormat> {
		match self {
			Format::Dash(_) => None,
			Format::Legacy(legacy) => Some(legacy.clone()),
			Format::Audio(_) => None,
		}
	}
}

impl From<(Vec<AdaptiveFormat>, Vec<LegacyFormat>)> for Formats {
	fn from(formats_tuple: (Vec<AdaptiveFormat>, Vec<LegacyFormat>)) -> Self {
		let mut video_formats: Vec<VideoFormat> = Vec::new();
		let mut audio_formats: Vec<AudioFormat> = Vec::new();
		let mut legacy_formats: Vec<LegacyFormat> = Vec::new();

		formats_tuple.0.into_iter().for_each(|adaptive_format| {
			if let Ok(audio) = AudioFormat::try_from(adaptive_format.clone()) {
				audio_formats.push(audio);
			} else if let Ok(video) = VideoFormat::try_from(adaptive_format) {
				video_formats.push(video);
			}
		});

		legacy_formats = formats_tuple.1;

		Formats { video_formats, audio_formats, legacy_formats }
	}
}
