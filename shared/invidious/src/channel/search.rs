use crate::{hidden::*, common::*};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use crate::fetch::{fetch, FetchError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSearch {
    pub items: Vec<SearchItem>,
}

impl ChannelSearch {
    fn url(server: &str, args: &str) -> String {
        format!("{}/api/v1/channels/search/{}", server, args)
    }

    async fn search(server: &str, args: &str) -> Result<Self, FetchError> {
        let search_url: String = Self::url(server, args);
        let search_json: String = fetch(&search_url).await?;
        let search: Self = serde_json::from_str(&search_json)?;
        Ok(search)
    }
}
