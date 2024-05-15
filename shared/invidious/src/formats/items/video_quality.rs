use std::{fmt, str::FromStr};

use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum VideoQuality {
	#[serde(rename = "144p")]
	_144p,
	#[serde(rename = "144p60")]
	_144p60,
	#[serde(rename = "240p")]
	_240p,
	#[serde(rename = "240p60")]
	_240p60,
	#[serde(rename = "360p")]
	_360p,
	#[serde(rename = "360p60")]
	_360p60,
	#[serde(rename = "480p")]
	_480p,
	#[serde(rename = "480p60")]
	_480p60,
	#[serde(rename = "720p")]
	_720p,
	#[serde(rename = "720p60")]
	_720p60,
	#[serde(rename = "1080p")]
	_1080p,
	#[serde(rename = "1080p60")]
	_1080p60,
	#[serde(rename = "1440p")]
	_1440p,
	#[serde(rename = "1440p60")]
	_1440p60,
	#[serde(rename = "2160p")]
	_2160p,
	#[serde(rename = "2160p60")]
	_2160p60,
	#[serde(rename = "4320p")]
	_4320p,
	#[serde(rename = "4320p60")]
	_4320p60,
}

impl fmt::Display for VideoQuality {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			VideoQuality::_144p => write!(f, "144p"),
			VideoQuality::_144p60 => write!(f, "144p60"),
			VideoQuality::_240p => write!(f, "240p"),
			VideoQuality::_240p60 => write!(f, "240p60"),
			VideoQuality::_360p => write!(f, "360p"),
			VideoQuality::_360p60 => write!(f, "360p60"),
			VideoQuality::_480p => write!(f, "480p"),
			VideoQuality::_480p60 => write!(f, "480p60"),
			VideoQuality::_720p => write!(f, "720p"),
			VideoQuality::_720p60 => write!(f, "720p60"),
			VideoQuality::_1080p => write!(f, "1080p"),
			VideoQuality::_1080p60 => write!(f, "1080p60"),
			VideoQuality::_1440p => write!(f, "1440p"),
			VideoQuality::_1440p60 => write!(f, "1440p60"),
			VideoQuality::_2160p => write!(f, "2160p"),
			VideoQuality::_2160p60 => write!(f, "2160p60"),
			VideoQuality::_4320p => write!(f, "4320p"),
			VideoQuality::_4320p60 => write!(f, "4320p60"),
		}
	}
}

impl FromStr for VideoQuality {
	type Err = RustyTubeError;

	fn from_str(quality_string: &str) -> Result<Self, Self::Err> {
		match quality_string {
			"144p" => Ok(VideoQuality::_144p),
			"240p" => Ok(VideoQuality::_240p),
			"360p" => Ok(VideoQuality::_360p),
			"480p" => Ok(VideoQuality::_480p),
			"720p" => Ok(VideoQuality::_720p),
			"720p60" => Ok(VideoQuality::_720p60),
			"1080p" => Ok(VideoQuality::_1080p),
			"1080p60" => Ok(VideoQuality::_1080p60),
			"1440p" => Ok(VideoQuality::_1440p),
			"1440p60" => Ok(VideoQuality::_1440p60),
			"2160p" => Ok(VideoQuality::_2160p),
			"2160p60" => Ok(VideoQuality::_2160p60),
			"4320p" => Ok(VideoQuality::_4320p),
			"4320p60" => Ok(VideoQuality::_4320p60),
			_ => Err(RustyTubeError::NoVideoQuality),
		}
	}
}
