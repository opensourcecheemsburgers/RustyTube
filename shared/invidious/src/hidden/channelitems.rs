use serde::{Deserialize, Serialize};

use crate::common::CommonImage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelatedChannel {
	#[serde(rename(serialize = "author", deserialize = "author"))]
	pub name: String,
	#[serde(rename(serialize = "authorId", deserialize = "authorId"))]
	pub id: String,
	#[serde(rename(serialize = "authorUrl", deserialize = "authorUrl"))]
	pub url: String,
	#[serde(rename(
		serialize = "authorThumbnails",
		deserialize = "authorThumbnails"
	))]
	pub thumbnails: Vec<CommonImage>,
}
