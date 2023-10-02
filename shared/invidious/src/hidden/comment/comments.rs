use serde::{Deserialize, Serialize};
use rustytube_error::RustyTubeError;
use crate::{Comment, fetch::fetch};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comments {
    #[serde(rename = "commentCount")]
    pub comment_count: Option<u32>,
    #[serde(rename = "videoId")]
    pub id: String,
    pub comments: Vec<Comment>,
    pub continuation: Option<String>,
}

impl Comments {
    fn url(server: &str, id: &str) -> String {
        format!("{server}/api/v1/comments/{id}")
    }

    pub async fn fetch_comments(server: &str, id: &str, continuation: Option<&str>) -> Result<Self, RustyTubeError> {
        let comments_url = match continuation {
            Some(continuation) => format!("{server}/api/v1/comments/{id}?continuation={continuation}"),
            None => format!("{server}/api/v1/comments/{id}"),
        };
        let comments_json = fetch(&comments_url).await?;
        let comments: Self = serde_json::from_str(&comments_json)?;
        Ok(comments)
    }
}
