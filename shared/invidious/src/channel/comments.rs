use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::fetch::fetch;
use crate::hidden::Comment;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelComments {
	#[serde(rename = "authorId")]
	pub author_id: String,
	pub comments: Vec<Comment>,
	#[serde(default)]
	pub content: String,
	pub continuation: Option<String>,
}

impl ChannelComments {
	async fn fetch_comments(
		server: &str,
		id: &str,
	) -> Result<Self, RustyTubeError> {
		let comments_url = format!("{server}/api/v1/channels/{id}/community?");
		let comments_json = fetch(&comments_url).await?;
		let comments = serde_json::from_str::<Self>(&comments_json)?;
		Ok(comments)
	}
}
