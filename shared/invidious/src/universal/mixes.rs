use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{fetch::fetch, hidden::MixVideo};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mix {
	pub title: String,
	#[serde(rename = "midId")]
	pub id: String,
	pub videos: Vec<MixVideo>,
}

impl Mix {
	async fn fetch_mix(server: &str, args: &str, lang: &str) -> Result<Self, RustyTubeError> {
		let mix_url: String = format!("{}/api/v1/mixes/{}", server, args);
		let mix_json: String = fetch(&mix_url).await?;
		let mix: Self = serde_json::from_str(&mix_json)?;
		Ok(mix)
	}
}
