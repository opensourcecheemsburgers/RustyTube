use crate::hidden::*;
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use crate::fetch::fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSearch {
    pub items: Vec<SearchResult>,
}

impl ChannelSearch {
    fn url(server: &str, args: &str) -> String {
        format!("{}/api/v1/channels/search/{}/", server, args)
    }

    async fn search(server: &str, args: &str) -> Result<Self, RustyTubeError> {
        let search_url: String = Self::url(server, args);
        let search_json: String = fetch(&search_url).await?;
        let search: Self = serde_json::from_str(&search_json)?;
        Ok(search)
    }
}
