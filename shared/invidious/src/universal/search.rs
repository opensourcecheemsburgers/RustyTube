use std::{
	fmt::{self, Display, Formatter},
	str::FromStr,
};

use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{fetch::fetch, hidden::SearchResult};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResults {
	pub items: Vec<SearchResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
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
	#[must_use]
	pub fn from_query_str(query: String) -> Self {
		Self { query, ..Default::default() }
	}

	#[must_use]
	pub const fn new(
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

		if let Some(timespan) = self.timespan {
			url.push_str(&format!("&date={timespan}"));
		}
		if let Some(duration) = self.duration {
			url.push_str(&format!("&duration={duration}"));
		}
		if let Some(response_type) = self.response_type {
			url.push_str(&format!("&type={response_type}"));
		}
		if let Some(features) = &self.features {
			let mut features_string = String::from("&features=");
			features.iter().enumerate().for_each(|(index, feature)| {
				if index < features.len() {
					features_string.push_str(&format!("{feature},"));
				} else {
					features_string.push_str(&feature.to_string());
				}
			});
			url.push_str(&features_string);
		}
		url
	}
}

impl SearchResults {
	pub async fn fetch_search_results(
		server: &str,
		args: &SearchArgs,
		page_number: u32,
		lang: &str,
	) -> Result<Self, RustyTubeError> {
		let url = format!(
			"{}/api/v1/search{}&page={}&hl={}",
			server,
			args.to_url(),
			page_number,
			lang
		);
		let search_json = fetch(&url).await?;
		let items: Vec<SearchResult> = serde_json::from_str(&search_json)?;
		Ok(Self { items })
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
	Relevance,
	Rating,
	Date,
	Views,
}

impl Display for Sort {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Relevance => write!(f, "relevance"),
			Self::Rating => write!(f, "rating"),
			Self::Date => write!(f, "date"),
			Self::Views => write!(f, "views"),
		}
	}
}

impl FromStr for Sort {
	type Err = RustyTubeError;

	fn from_str(duration_str: &str) -> Result<Self, Self::Err> {
		match duration_str {
			"relevance" => Ok(Self::Relevance),
			"rating" => Ok(Self::Rating),
			"date" => Ok(Self::Date),
			"views" => Ok(Self::Views),
			_ => Err(RustyTubeError::SearchArgs),
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSpan {
	Hour,
	Day,
	Week,
	Month,
	Year,
}

impl Display for TimeSpan {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Hour => write!(f, "hour"),
			Self::Day => write!(f, "day"),
			Self::Week => write!(f, "week"),
			Self::Month => write!(f, "month"),
			Self::Year => write!(f, "year"),
		}
	}
}

impl FromStr for TimeSpan {
	type Err = RustyTubeError;

	fn from_str(duration_str: &str) -> Result<Self, Self::Err> {
		match duration_str {
			"hour" => Ok(Self::Hour),
			"day" => Ok(Self::Day),
			"week" => Ok(Self::Week),
			"month" => Ok(Self::Month),
			"year" => Ok(Self::Year),
			_ => Err(RustyTubeError::SearchArgs),
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Duration {
	Short,
	Long,
	Medium,
}

impl Display for Duration {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Short => write!(f, "short"),
			Self::Long => write!(f, "long"),
			Self::Medium => write!(f, "medium"),
		}
	}
}

impl FromStr for Duration {
	type Err = RustyTubeError;

	fn from_str(duration_str: &str) -> Result<Self, Self::Err> {
		match duration_str {
			"short" => Ok(Self::Short),
			"long" => Ok(Self::Long),
			"medium" => Ok(Self::Medium),
			_ => Err(RustyTubeError::SearchArgs),
		}
	}
}

#[derive(
	Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default,
)]
pub enum ResponseType {
	Video,
	Playlist,
	Channel,
	Movie,
	Show,
	#[default]
	All,
}

impl Display for ResponseType {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Video => write!(f, "video"),
			Self::Playlist => write!(f, "playlist"),
			Self::Channel => write!(f, "channel"),
			Self::Movie => write!(f, "movie"),
			Self::Show => write!(f, "show"),
			Self::All => write!(f, "all"),
		}
	}
}

impl FromStr for ResponseType {
	type Err = RustyTubeError;

	fn from_str(response_type_str: &str) -> Result<Self, Self::Err> {
		match response_type_str {
			"video" => Ok(Self::Video),
			"playlist" => Ok(Self::Playlist),
			"channel" => Ok(Self::Channel),
			"movie" => Ok(Self::Movie),
			"show" => Ok(Self::Show),
			"all" => Ok(Self::All),
			_ => Err(RustyTubeError::SearchArgs),
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
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
	VirtualReality180,
}

impl Display for Feature {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::HighDefinition => write!(f, "hd"),
			Self::Subtitles => write!(f, "subtitles"),
			Self::CreativeCommons => write!(f, "creative_commons"),
			Self::_3D => write!(f, "3d"),
			Self::Live => write!(f, "live"),
			Self::Purchased => write!(f, "purchased"),
			Self::_4K => write!(f, "4k"),
			Self::_360Degrees => write!(f, "360"),
			Self::Location => write!(f, "location"),
			Self::HighDynamicRange => write!(f, "hdr"),
			Self::VirtualReality180 => write!(f, "vr180"),
		}
	}
}

// Self::HighDefinition => String::from("hd"),
// Self::Subtitles => String::from("subtitles"),
// Self::CreativeCommons => String::from("creative_commons"),
// Self::_3D => String::from("3d"),
// Self::Live => String::from("live"),
// Self::Purchased => String::from("purchased"),
// Self::_4K => String::from("4k"),
// Self::_360Degrees => String::from("360"),
// Self::Location => String::from("location"),
// Self::HighDynamicRange => String::from("hdr"),
// Self::VirtualReality180 => String::from("vr180"),
impl FromStr for Feature {
	type Err = RustyTubeError;

	fn from_str(feature_str: &str) -> Result<Self, Self::Err> {
		match feature_str {
			"hd" => Ok(Self::HighDefinition),
			"subtitles" => Ok(Self::Subtitles),
			"creative_commons" => Ok(Self::CreativeCommons),
			"3d" => Ok(Self::_3D),
			"live" => Ok(Self::Live),
			"purchased" => Ok(Self::Purchased),
			"4k" => Ok(Self::_4K),
			"360" => Ok(Self::_360Degrees),
			"location" => Ok(Self::Location),
			"hdr" => Ok(Self::HighDynamicRange),
			"vr180" => Ok(Self::VirtualReality180),
			_ => Err(RustyTubeError::SearchArgs),
		}
	}
}
