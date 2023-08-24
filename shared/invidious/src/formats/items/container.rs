use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Container {
	#[serde(rename = "m4a")]
	M4A,
	#[serde(rename = "webm")]
	WEBM,
	#[serde(rename = "mp4")]
	MP4,
	#[serde(rename = "3gp")]
	_3GP,
}

impl fmt::Display for Container {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Container::M4A => write!(f, "m4a"),
			Container::WEBM => write!(f, "webm"),
			Container::MP4 => write!(f, "mp4"),
			Container::_3GP => write!(f, "3gp"),
		}
	}
}

impl Default for Container {
	fn default() -> Self {
		Self::MP4
	}
}