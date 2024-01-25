use invidious::{AudioFormat, Container, DashFormat, Format, Formats, LegacyFormat, VideoFormat};
use leptos::*;
use rustytube_error::RustyTubeError;

use crate::{contexts::PlayerConfigCtx, utils::is_webkit};

pub fn get_format(formats: &Formats) -> Result<Format, RustyTubeError> {
	let audio_format = find_audio_format(&formats)?;
	let video_format = find_video_format(&formats);

	let format = match video_format {
		Ok(video_format) => Format::Dash(DashFormat::new(video_format, audio_format)),
		Err(_) => match find_legacy_format(&formats) {
			Ok(legacy_format) => Format::Legacy(legacy_format),
			Err(_) => Format::Audio(audio_format),
		},
	};
	Ok(format)
}

pub fn find_video_format(formats: &Formats) -> Result<VideoFormat, RustyTubeError> {
	let default_video_quality =
		move || expect_context::<PlayerConfigCtx>().0 .0.get().default_video_quality;

	let preferred_format =
		formats.video_formats.iter().find(|x| x.quality_label == default_video_quality()).cloned();

	match preferred_format {
		Some(_) => preferred_format,
		None => formats.video_formats.first().cloned(),
	}
	.ok_or(RustyTubeError::no_dash_video_format_available())
}

pub fn find_legacy_format(formats: &Formats) -> Result<LegacyFormat, RustyTubeError> {
	formats.legacy_formats.last().cloned().ok_or(RustyTubeError::no_legacy_format_available())
}

pub fn find_audio_format(formats: &Formats) -> Result<AudioFormat, RustyTubeError> {
	let config = expect_context::<PlayerConfigCtx>().0 .0;

	let audio_formats = match is_webkit() {
		true => filter_mp4_audio_formats(&formats.audio_formats),
		false => formats.audio_formats.clone(),
	};

	let preferred_format = audio_formats
		.iter()
		.find(|format| format.audio_quality == config.get().default_audio_quality)
		.cloned();

	match preferred_format {
		Some(_) => preferred_format,
		None => audio_formats.first().cloned(),
	}
	.ok_or(RustyTubeError::no_audio_format_available())
}

pub fn filter_mp4_audio_formats(formats: &Vec<AudioFormat>) -> Vec<AudioFormat> {
	formats
		.iter()
		.filter_map(|format| {
			let a = format.r#type.contains("mp4");
			let b = format.clone().container.map(|container| (container.eq(&Container::M4A)));

			(a && b.unwrap_or_default()).then(|| format.clone())
		})
		.collect::<Vec<AudioFormat>>()
}
