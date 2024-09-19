use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{fetch::fetch, hidden::PopularItem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Popular {
	pub items: Vec<PopularItem>,
}

impl Popular {
	pub async fn fetch_popular(
		server: &str,
		lang: &str,
	) -> Result<Self, RustyTubeError> {
		let url = format!("{server}/api/v1/popular?hl={lang}");
		let popular_json = fetch(&url).await?;
		let items: Vec<PopularItem> = serde_json::from_str(&popular_json)?;
		Ok(Self { items })
	}
}
