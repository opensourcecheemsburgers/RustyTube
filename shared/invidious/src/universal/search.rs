use serde::{Deserialize, Serialize};
use crate::hidden::{CountryCode, SearchItem};
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::error::RustyTubeError;
use crate::fetch::fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Search {
    pub items: Vec<SearchItem>,
}

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

impl Search {
    fn url(server: &str, args: &SearchArgs) -> String {
        let mut url: String = format!("{}/api/v1/search/?q={}&sort_by=", server, args.query);

        if let Some(timespan) = &args.timespan {
            url.push_str(&format!("?date={timespan}"))
        }
        if let Some(duration) = &args.duration {
            url.push_str(&format!("?duration={duration}"))
        }
        if let Some(response_type) = &args.response_type {
            url.push_str(&format!("?duration={response_type}"))
        }
        if let Some(features) = &args.features {
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

    pub async fn search(server: &str, args: &SearchArgs) -> Result<Search, RustyTubeError> {
        let url = Self::url(server, args);
        let search_json = fetch(&url).await?;
        let items: Vec<SearchItem> = serde_json::from_str(&search_json)?;
        Ok(Search { items })
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Clone)]
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