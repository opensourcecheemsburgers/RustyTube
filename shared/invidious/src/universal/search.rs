use serde::{Deserialize, Serialize};
use crate::hidden::SearchResult;
use std::str::FromStr;
use rustytube_error::RustyTubeError;
use crate::fetch::fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResults {
    pub items: Vec<SearchResult>,
}


#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SearchArgs {
    page: Option<u32>,
    pub query: String,
    pub sort: Option<Sort>,
    pub timespan: Option<TimeSpan>,
    pub duration: Option<Duration>,
    pub response_type: Option<ResponseType>,
    pub features: Option<Vec<Feature>>,
    // pub region: Option<CountryCode>,
}

impl SearchArgs {
    pub fn from_str(query: String) -> Self {
        Self { query, ..Default::default()}
    }

    pub fn new(
        query: String,
        sort: Option<Sort>, 
        timespan: Option<TimeSpan>,
        duration: Option<Duration>, 
        response_type: Option<ResponseType>, 
        features: Option<Vec<Feature>>, 
        // region: Option<CountryCode>
    ) -> Self {
        let page = Some(1);

        Self { page, query, sort, timespan, duration, response_type, features }
    }

    pub fn to_url(&self) -> String {
            let mut url: String = format!("?q={}", self.query);
    
            self.timespan.map(|timespan| url.push_str(&format!("&date={}", timespan.to_string())));
            self.duration.map(|duration| url.push_str(&format!("&duration={}", duration.to_string())));
            self.response_type.map(|response_type| url.push_str(&format!("&type={}", response_type.to_string())));
            self.features.clone().map(|features| {
                let mut features_string = String::from("&features=");
                features.iter().enumerate().for_each(|(index, feature)| {
                    match index < features.len() {
                        true => features_string.push_str(&format!("{},",feature.to_string())),
                        false => features_string.push_str(&format!("{}", feature.to_string()))
                    }
                });
                url.push_str(&features_string);
            });
            url
        }
}

impl SearchResults {
    pub async fn fetch_search_results(server: &str, args: SearchArgs, page_number: u32) -> Result<Self, RustyTubeError> {
        let url = format!("{}/api/v1/search{}&page={}", server, args.to_url(), page_number);
        let search_json = fetch(&url).await?;
        let items: Vec<SearchResult> = serde_json::from_str(&search_json)?;
        Ok(Self { items })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Sort {
    Relevance,
    Rating,
    Date,
    Views
}

impl ToString for Sort {
    fn to_string(&self) -> String {
        match self {
            Sort::Relevance => String::from("relevance"),
            Sort::Rating => String::from("rating"),
            Sort::Date => String::from("date"),
            Sort::Views => String::from("views"),
        }
    }
}

impl FromStr for Sort {
    type Err = RustyTubeError;
    
    fn from_str(duration_str: &str) -> Result<Self, Self::Err> {
        match duration_str {
            "relevance" => Ok(Sort::Relevance),
            "rating" => Ok(Sort::Rating),
            "date" => Ok(Sort::Date),
            "views" => Ok(Sort::Views),
            _ => Err(RustyTubeError::search_url_parse())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum TimeSpan {
    Hour,
    Day,
    Week,
    Month,
    Year
}

impl ToString for TimeSpan {
    fn to_string(&self) -> String {
        match self {
            TimeSpan::Hour => String::from("hour"),
            TimeSpan::Day => String::from("day"),
            TimeSpan::Week => String::from("week"),
            TimeSpan::Month => String::from("month"),
            TimeSpan::Year => String::from("year"),
        }
    }
}

impl FromStr for TimeSpan {
    type Err = RustyTubeError;
    
    fn from_str(duration_str: &str) -> Result<Self, Self::Err> {
        match duration_str {
            "hour" => Ok(TimeSpan::Hour),
            "day" => Ok(TimeSpan::Day),
            "week" => Ok(TimeSpan::Week),
            "month" => Ok(TimeSpan::Month),
            "year" => Ok(TimeSpan::Year),
            _ => Err(RustyTubeError::search_url_parse())
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Duration {
    Short,
    Long,
    Medium
}

impl ToString for Duration {
    fn to_string(&self) -> String {
        match self {
            Duration::Short => String::from("short"),
            Duration::Long => String::from("long"),
            Duration::Medium => String::from("medium"),
        }
    }
}

impl FromStr for Duration {
    type Err = RustyTubeError;
    
    fn from_str(duration_str: &str) -> Result<Self, Self::Err> {
        match duration_str {
            "short" => Ok(Duration::Short),
            "long" => Ok(Duration::Long),
            "medium" => Ok(Duration::Medium),
            _ => Err(RustyTubeError::search_url_parse())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Default)]
pub enum ResponseType {
    Video,
    Playlist,
    Channel,
    Movie,
    Show,
    #[default]
    All
}

impl ToString for ResponseType {
    fn to_string(&self) -> String {
        match self {
            ResponseType::Video => String::from("video"),
            ResponseType::Playlist => String::from("playlist"),
            ResponseType::Channel => String::from("channel"),
            ResponseType::Movie => String::from("movie"),
            ResponseType::Show => String::from("show"),
            ResponseType::All => String::from("all"),
        }
    }
}

impl FromStr for ResponseType {
    type Err = RustyTubeError;

    fn from_str(response_type_str: &str) -> Result<Self, Self::Err> {
        match response_type_str {
            "video" => Ok(ResponseType::Video),
            "playlist" => Ok(ResponseType::Playlist),
            "channel"=> Ok(ResponseType::Channel), 
            "movie" => Ok(ResponseType::Movie),
            "show" => Ok(ResponseType::Show), 
            "all" => Ok(ResponseType::All),
            _ => Err(RustyTubeError::search_url_parse())
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
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

impl ToString for Feature {
    fn to_string(&self) -> String {
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

impl FromStr for Feature {
    type Err = RustyTubeError;

    fn from_str(feature_str: &str) -> Result<Self, Self::Err> {
        match feature_str {
            "hd" => Ok(Feature::HighDefinition),
            "subtitles" => Ok(Feature::Subtitles),
            "creative_commons" => Ok(Feature::CreativeCommons),
            "3d" => Ok(Feature::_3D) ,
            "live" => Ok(Feature::Live),
            "purchased" => Ok(Feature::Purchased), 
            "4k" => Ok(Feature::_4K),
            "360" => Ok(Feature::_360Degrees) ,
            "location" => Ok(Feature::Location), 
            "hdr" => Ok(Feature::HighDynamicRange) ,
            "vr180" => Ok(Feature::VirtualReality180),
            _ => Err(RustyTubeError::search_url_parse())
        }
    }
}














































































