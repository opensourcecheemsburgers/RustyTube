use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{fetch::fetch, hidden::SearchResult};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSearch {
	pub items: Vec<SearchResult>,
}

impl ChannelSearch {
	async fn search(server: &str, args: &str) -> Result<Self, RustyTubeError> {
		let search_url = format!("{server}/api/v1/channels/search/{args}/");
		let search_json: String = fetch(&search_url).await?;
		let search: Self = serde_json::from_str(&search_json)?;
		Ok(search)
	}
}
