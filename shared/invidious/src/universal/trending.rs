use serde::{Deserialize, Serialize};
use crate::common::CommonVideo;
use rustytube_error::RustyTubeError;
use crate::fetch::fetch;
use crate::hidden::CountryCode;

#[derive(PartialEq, Clone)]
pub enum TrendingCategory {
    Default,
    Music,
    Gaming,
    Movies
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trending {
    pub videos: Vec<CommonVideo>,
}

impl Trending {
    fn url(server: &str, category: TrendingCategory, region: CountryCode) -> String {
        match category {
            TrendingCategory::Default => format!("{server}/api/v1/trending/?region={region}"),
            TrendingCategory::Music => format!("{server}/api/v1/trending?type=music&region={region}"),
            TrendingCategory::Gaming => format!("{server}/api/v1/trending?type=gaming&region={region}"),
            TrendingCategory::Movies => format!("{server}/api/v1/trending?type=movies")
        }
    }

    pub async fn fetch_trending(server: &str, category: TrendingCategory, region: CountryCode) -> Result<Self, RustyTubeError> {
        let url = Self::url(server, category, region);
        let trending_json = fetch(&url).await?;
        let videos: Vec<CommonVideo> = serde_json::from_str(&trending_json)?;
        Ok(Trending { videos })
    }
}
