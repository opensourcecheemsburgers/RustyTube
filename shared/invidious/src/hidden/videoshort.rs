use serde::{Deserialize, Serialize};

use crate::common::CommonThumbnail;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoShort {
	#[serde(rename = "videoId")]
	pub id: String,
	pub title: String,
	#[serde(rename = "videoThumbnails")]
	pub thumbnails: Vec<CommonThumbnail>,
	pub author: String,
	#[serde(rename = "lengthSeconds")]
	pub length: u32,
	#[serde(rename = "viewCountText")]
	pub views_text: String,
}

impl PartialEq for VideoShort {
	fn eq(&self, other: &Self) -> bool {
		self.id.eq_ignore_ascii_case(&other.id)
	}
}
