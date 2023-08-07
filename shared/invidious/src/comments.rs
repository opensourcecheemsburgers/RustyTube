use serde::{Deserialize, Serialize};
use crate::fetch::{fetch, FetchError};
use crate::hidden::Comment;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comments {
    #[serde(rename = "commentCount")]
    pub comment_count: u32,
    #[serde(rename = "videoId")]
    pub id: String,
    pub comments: Vec<Comment>,
    pub continuation: Option<String>,
}

impl Comments {
    fn url(server: &str, args: &str) -> String {
        format!("{server}/api/v1/comments/{args}")
    }

    pub async fn fetch_comments(server: &str, id: &str, args: Option<&str>) -> Result<Self, FetchError> {
        let comments_url: String = Self::url(server, id);
        let comments_json: String = fetch(&comments_url).await?;
        let comments: Self = serde_json::from_str(&comments_json)?;
        Ok(comments)
    }
}