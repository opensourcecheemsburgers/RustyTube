use gloo::history::query;
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Suggestions {
    pub query: String,
    pub suggestions: Vec<String>,
}

impl Suggestions {
    pub async fn fetch_suggestions(query: &str, server: &str) -> Result<Suggestions, RustyTubeError> {
        let url = format!("{}/api/v1/search/suggestions?q={}", server, query);
        let suggestions_json = fetch(&url).await?;
        let suggestions = serde_json::from_str(&suggestions_json)?;
        Ok(suggestions)
    }
}

