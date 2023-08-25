use serde::{Deserialize, Serialize};
use rustytube_error::RustyTubeError;
use crate::fetch::fetch;
use crate::hidden::PopularItem;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Popular {
    pub items: Vec<PopularItem>,
}

impl Popular {
    fn url(server: &str, args: &str) -> String {
        format!("{server}/api/v1/popular/{args}")
    }

    pub async fn fetch_popular(server: &str) -> Result<Self, RustyTubeError> {
        let url = Self::url(server, "");
        let popular_json = fetch(&url).await?;
        let items: Vec<PopularItem> = serde_json::from_str(&popular_json)?;
        Ok(Popular { items })
    }
}
