use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{fetch::fetch, hidden::Caption};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Captions {
	pub captions: Vec<Caption>,
}

impl Captions {
	pub async fn fetch_captions(
		server: &str,
		id: &str,
	) -> Result<Self, RustyTubeError> {
		let captions_url: String = format!("{server}/api/v1/captions/{id}");
		let captions_json: String = fetch(&captions_url).await?;
		let captions: Self = serde_json::from_str(&captions_json)?;
		Ok(captions)
	}
}
