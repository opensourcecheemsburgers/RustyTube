use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Caption {
	pub label: String,
	#[serde(rename = "language_code")]
	#[serde(default)]
	pub language: String,
	pub url: String,
}

impl Caption {
	pub fn url(&self, server: &str) -> String {
		format!("{}{}", server, self.url)
	}
}

impl PartialEq for Caption {
	fn eq(&self, other: &Self) -> bool {
		self.url.eq_ignore_ascii_case(&other.url)
	}
}
