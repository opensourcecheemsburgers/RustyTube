use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{common::CommonImage, fetch};

#[derive(Clone, Eq, Deserialize, Serialize, Debug)]
pub struct Comment {
	#[serde(default)]
	pub verified: bool,
	#[serde(rename = "commentId")]
	pub id: String,
	#[serde(rename = "likeCount")]
	pub likes: u32,
	#[serde(rename = "isEdited")]
	pub edited: bool,
	pub content: String,
	#[serde(rename = "contentHtml")]
	pub content_html: String,
	pub published: u64,
	#[serde(rename = "publishedText")]
	pub published_text: String,

	pub author: String,
	#[serde(rename = "authorThumbnails")]
	pub author_thumbnails: Vec<CommonImage>,
	#[serde(rename = "authorId")]
	pub author_id: String,
	#[serde(rename = "authorUrl")]
	pub author_url: String,

	#[serde(rename = "authorIsChannelOwner")]
	pub channel_owner: bool,
	#[serde(default)]
	#[serde(rename = "creatorHeart")]
	pub heart: Option<CreatorHeart>,
	#[serde(default)]
	#[serde(rename = "replies")]
	pub replies_info: Option<RepliesInfo>,
}

impl PartialEq for Comment {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Debug)]
pub struct CreatorHeart {
	#[serde(rename = "creatorThumbnail")]
	pub thumbnail: String,
	#[serde(rename = "creatorName")]
	pub name: String,
}

#[derive(Clone, Eq, Deserialize, Serialize, Debug)]
pub struct RepliesInfo {
	#[serde(rename = "replyCount")]
	pub replies: u32,
	pub continuation: String,
}

impl PartialEq for RepliesInfo {
	fn eq(&self, other: &Self) -> bool {
		self.continuation.eq_ignore_ascii_case(&other.continuation)
	}
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct Replies {
	#[serde(rename = "videoId")]
	pub id: String,
	pub comments: Vec<Comment>,
	pub continuation: Option<String>,
}

impl Replies {
	pub async fn fetch_replies(
		server: &str,
		id: &str,
		continuation: &str,
		lang: &str,
	) -> Result<Self, RustyTubeError> {
		let comments_url =
			format!("{server}/api/v1/comments/{id}?continuation={continuation}&hl={lang}");
		let comments_json = fetch(&comments_url).await?;
		let replies = serde_json::from_str::<Self>(&comments_json)?;
		Ok(replies)
	}
}
