use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::fetch;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dislikes {
	id: String,
	date_created: String,
	likes: u64,
	dislikes: u64,
	rating: f64,
	view_count: u64,
	deleted: bool,
}

impl Dislikes {
	pub async fn fetch_dislikes(id: &str) -> Result<u64, RustyTubeError> {
		let url =
			format!("https://returnyoutubedislikeapi.com/votes?videoId={id}");
		let dislike_info = serde_json::from_str::<Self>(&fetch(&url).await?)?;
		Ok(dislike_info.dislikes)
	}
}
