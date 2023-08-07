use serde::{Deserialize, Serialize};
use crate::fetch::{fetch, FetchError};
use crate::hidden::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Captions {
    pub captions: Vec<Caption>,
}

impl Captions {
    fn url(server: &str, args: &str) -> String {
        format!("{server}/api/v1/captions/{args}")
    }

    async fn fetch_captions(server: &str, id: &str) -> Result<Self, FetchError> {
        let captions_url: String = Self::url(server, id);
        let captions_json: String = fetch(&captions_url).await?;
        let captions: Captions = serde_json::from_str(&captions_json)?;
        Ok(captions)
    }
}
