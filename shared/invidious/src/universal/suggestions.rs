use html_escape::decode_html_entities;
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Suggestions {
	pub query: String,
	pub suggestions: Vec<String>,
}

impl Suggestions {
	pub async fn fetch_suggestions(
		query: &str,
		server: &str,
		lang: &str,
	) -> Result<Self, RustyTubeError> {
		let url =
			format!("{server}/api/v1/search/suggestions?q={query}&hl={lang}");
		let suggestions_json = fetch(&url).await?;
		let mut suggestions = serde_json::from_str::<Self>(&suggestions_json)?;
		let decoded_suggestions = suggestions
			.suggestions
			.into_iter()
			.map(|suggestion| decode_html_entities(&suggestion).to_string())
			.collect::<Vec<String>>();
		suggestions.suggestions = decoded_suggestions;

		Ok(suggestions)
	}
}
