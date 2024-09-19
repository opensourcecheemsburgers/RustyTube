use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{fetch::fetch, hidden::MixVideo};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mix {
	pub title: String,
	#[serde(rename = "mixId")]
	pub id: String,
	pub videos: Vec<MixVideo>,
}

impl Mix {
	async fn fetch_mix(
		server: &str,
		lang: &str,
	) -> Result<Self, RustyTubeError> {
		let mix_url = format!("{server}/api/v1/mixes?hl={lang}");
		let mix_json = fetch(&mix_url).await?;
		let mix = serde_json::from_str::<Self>(&mix_json)?;
		Ok(mix)
	}
}
