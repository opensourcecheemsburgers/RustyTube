use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
			Self::M4A => write!(f, "m4a"),
			Self::WEBM => write!(f, "webm"),
			Self::MP4 => write!(f, "mp4"),
			Self::_3GP => write!(f, "3gp"),
		}
	}
}

impl Default for Container {
	fn default() -> Self {
		Self::MP4
	}
}
