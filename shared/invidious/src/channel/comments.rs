use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use crate::fetch::fetch;
use crate::Comment;

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
    fn url(server: &str, args: &str) -> String {
        format!("{server}/api/v1/channels/{args}/community?",)
    }

    async fn fetch_comments(server: &str, id: &str, args: Option<&str>) -> Result<Self, RustyTubeError> {
        let comments_url: String = Self::url(server, id);
        let comments_json: String = fetch(&comments_url).await?;
        let comments: Self = serde_json::from_str(&comments_json)?;
        Ok(comments)
    }
}
