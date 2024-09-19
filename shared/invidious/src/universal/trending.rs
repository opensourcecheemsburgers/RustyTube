use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{common::CommonVideo, fetch::fetch};

#[derive(PartialEq, Eq, Clone)]
pub enum TrendingCategory {
	Default,
	Music,
	Gaming,
	Movies,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trending {
	pub videos: Vec<CommonVideo>,
}

impl Trending {
	fn url(
		server: &str,
		category: &TrendingCategory,
		region: &str,
		lang: &str,
	) -> String {
		match category {
			TrendingCategory::Default => {
				format!("{server}/api/v1/trending/?region={region}&hl={lang}")
			}
			TrendingCategory::Music => {
				format!("{server}/api/v1/trending?type=music&region={region}&hl={lang}")
			}
			TrendingCategory::Gaming => {
				format!("{server}/api/v1/trending?type=gaming&region={region}&hl={lang}")
			}
			TrendingCategory::Movies => {
				format!("{server}/api/v1/trending?type=movies&hl={lang}")
			}
		}
	}

	pub async fn fetch_trending(
		server: &str,
		category: &TrendingCategory,
		region: &str,
		lang: &str,
	) -> Result<Self, RustyTubeError> {
		let url = Self::url(server, category, region, lang);
		let trending_json = fetch(&url).await?;
		let videos: Vec<CommonVideo> = serde_json::from_str(&trending_json)?;
		Ok(Self { videos })
	}
}
