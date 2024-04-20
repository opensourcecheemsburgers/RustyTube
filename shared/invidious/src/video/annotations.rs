use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::fetch::fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Annotations {
	pub inner: String,
}

impl From<String> for Annotations {
	fn from(inner: String) -> Self {
		Self { inner }
	}
}

impl Annotations {
	fn url(server: &str, args: &str) -> String {
		format!("{server}/api/v1/annotations/{args}")
	}

	async fn fetch(server: &str, args: &str) -> Result<Self, RustyTubeError> {
		let annotations_url: String = Self::url(server, args);
		let annotations_json: String = fetch(&annotations_url).await?;
		let annotations: Self = serde_json::from_str(&annotations_json)?;
		Ok(annotations)
	}
}
