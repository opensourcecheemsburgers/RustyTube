use serde::{Deserialize, Serialize};
use crate::hidden::{CountryCode, SearchResult};
use std::fmt;
use std::fmt::{Display, Formatter};
use rustytube_error::RustyTubeError;
use crate::fetch::fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResults {
    pub items: Vec<SearchResult>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchArgs {
    pub page: u32,
    pub query: String,
    pub sort: Sort,
    pub timespan: Option<TimeSpan>,
    pub duration: Option<Duration>,
    pub response_type: Option<ResponseType>,
    pub features: Option<Vec<Feature>>,
    pub region: CountryCode,

}

impl SearchArgs {
    fn url(&self, server: &str) -> String {
        let mut url: String = format!("{}/api/v1/search/?q={}&sort_by=", server, self.query);

        if let Some(timespan) = self.timespan {
            url.push_str(&format!("?date={timespan}"))
        }
        if let Some(duration) = self.duration {
            url.push_str(&format!("?duration={duration}"))
        }
        if let Some(response_type) = self.response_type {
            url.push_str(&format!("?duration={response_type}"))
        }
        if let Some(features) = self.features.clone() {
            let mut features_string = String::from("?features=");
            features.iter().enumerate().for_each(|(index, feature)| {
                let feature_string: String = feature.clone().into();
                match index < features.len() {
                    true => features_string.push_str(&format!("{},",feature_string)),
                    false => features_string.push_str(&format!("{}", feature_string))
                }
            });
            url.push_str(&features_string);
        }

        url
    }
}

impl SearchResults {
    pub async fn search(url: &str) -> Result<Self, RustyTubeError> {
        let search_json = fetch(&url).await?;
        let items: Vec<SearchResult> = serde_json::from_str(&search_json)?;
        Ok(Self { items })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Sort {
    Relevance,
    Rating,
    Date,
    Views
}

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TimeSpan {
    Hour,
    Day,
    Week,
    Month,
    Year
}

impl Display for TimeSpan {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Duration {
    Short,
    Long,
    Medium
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ResponseType {
    Video,
    Playlist,
    Channel,
    Movie,
    Show,
    All
}

impl Display for ResponseType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Feature {
    HighDefinition,
    Subtitles,
    CreativeCommons,
    _3D,
    Live,
    Purchased,
    _4K,
    _360Degrees,
    Location,
    HighDynamicRange,
    VirtualReality180
}

impl Into<String> for Feature {
    fn into(self) -> String {
        match self {
            Feature::HighDefinition => String::from("hd"),
            Feature::Subtitles => String::from("subtitles"),
            Feature::CreativeCommons => String::from("creative_commons"),
            Feature::_3D => String::from("3d"),
            Feature::Live => String::from("live"),
            Feature::Purchased => String::from("purchased"),
            Feature::_4K => String::from("4k"),
            Feature::_360Degrees => String::from("360"),
            Feature::Location => String::from("location"),
            Feature::HighDynamicRange => String::from("hdr"),
            Feature::VirtualReality180 => String::from("vr180"),
        }
    }
}