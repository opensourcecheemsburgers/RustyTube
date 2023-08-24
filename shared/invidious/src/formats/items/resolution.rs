use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Resolution {
	#[serde(rename = "144p")]
	_144p,
	#[serde(rename = "240p")]
	_240p,
	#[serde(rename = "360p")]
	_360p,
	#[serde(rename = "480p")]
	_480p,
	#[serde(rename = "720p")]
	_720p,
	#[serde(rename = "1080p")]
	_1080p,
	#[serde(rename = "1440p")]
	_1440p,
	#[serde(rename = "2160p")]
	_2160p,
	#[serde(rename = "4320p")]
	_4320p,
}

impl fmt::Display for Resolution {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Resolution::_144p => write!(f, "144p"),
			Resolution::_240p => write!(f, "240p"),
			Resolution::_360p => write!(f, "360p"),
			Resolution::_480p => write!(f, "480p"),
			Resolution::_720p => write!(f, "720p"),
			Resolution::_1080p => write!(f, "1080p"),
			Resolution::_1440p => write!(f, "1440p"),
			Resolution::_2160p => write!(f, "2160p"),
			Resolution::_4320p => write!(f, "4320p"),
		}
	}
}