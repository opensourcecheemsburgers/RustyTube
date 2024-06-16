use crate::{AudioFormat, VideoFormat};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashFormat {
	pub video: VideoFormat,
	pub audio: AudioFormat,
}

impl DashFormat {
	pub fn new(video: VideoFormat, audio: AudioFormat) -> Self {
		Self { video, audio }
	}
}
