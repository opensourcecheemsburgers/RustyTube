use crate::formats::audio::AudioFormat;
use crate::formats::legacy::LegacyFormat;
use crate::formats::video::VideoFormat;
use crate::video::Video;

use super::{Container, AdaptiveFormat};

#[derive(Debug, Clone, PartialEq)]
pub struct Formats {
	pub video_formats: Vec<VideoFormat>,
	pub audio_formats: Vec<AudioFormat>,
	pub legacy_formats: Vec<LegacyFormat>
}

pub enum FormatCategory {
    Dash,
    Legacy,
    Audio
}

// pub enum Format {
//     Dash(VideoFormat),
//     Legacy(LegacyFormat),
//     Audio(AudioFormat)
// }

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