use std::{fmt, str::FromStr};
use serde::{Deserialize, Serialize};
use rustytube_error::RustyTubeError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum QualityLabel {
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
	_4320p60
}

impl fmt::Display for QualityLabel {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			QualityLabel::_144p => write!(f, "144p"),
			QualityLabel::_144p60 => write!(f, "144p60"),
			QualityLabel::_240p => write!(f, "240p"),
			QualityLabel::_240p60 => write!(f, "240p60"),
			QualityLabel::_360p => write!(f, "360p"),
			QualityLabel::_360p60 => write!(f, "360p60"),
			QualityLabel::_480p => write!(f, "480p"),
			QualityLabel::_480p60 => write!(f, "480p60"),
			QualityLabel::_720p => write!(f, "720p"),
			QualityLabel::_720p60 => write!(f, "720p60"),
			QualityLabel::_1080p => write!(f, "1080p"),
			QualityLabel::_1080p60 => write!(f, "1080p60"),
			QualityLabel::_1440p => write!(f, "1440p"),
			QualityLabel::_1440p60 => write!(f, "1440p60"),
			QualityLabel::_2160p => write!(f, "2160p"),
			QualityLabel::_2160p60 => write!(f, "2160p60"),
			QualityLabel::_4320p => write!(f, "4320p"),
			QualityLabel::_4320p60 => write!(f, "4320p60"),
		}
	}
}

impl FromStr for QualityLabel {
	type Err = RustyTubeError;

	fn from_str(quality_string: &str) -> Result<Self, Self::Err> {
		match quality_string {
			"144p" => Ok(QualityLabel::_144p),
			"240p" => Ok(QualityLabel::_240p),
			"360p" => Ok(QualityLabel::_360p),
			"480p" => Ok(QualityLabel::_480p),
			"720p" => Ok(QualityLabel::_720p),
			"720p60" => Ok(QualityLabel::_720p60),
			"1080p" => Ok(QualityLabel::_1080p),
			"1080p60" => Ok(QualityLabel::_1080p60),
			"1440p" => Ok(QualityLabel::_1440p),
			"1440p60" => Ok(QualityLabel::_1440p60),
			"2160p" => Ok(QualityLabel::_2160p),
			"2160p60" => Ok(QualityLabel::_2160p60),
			"4320p" => Ok(QualityLabel::_4320p),
			"4320p60" => Ok(QualityLabel::_4320p60),
			_ => Err(RustyTubeError::format_parse())
		}
	}
}