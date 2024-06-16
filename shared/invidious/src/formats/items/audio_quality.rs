use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AudioQuality {
	#[serde(rename = "AUDIO_QUALITY_ULTRALOW")]
	UltraLow,
	#[serde(rename = "AUDIO_QUALITY_LOW")]
	Low,
	#[serde(rename = "AUDIO_QUALITY_MEDIUM")]
	Medium,
	#[serde(rename = "AUDIO_QUALITY_HIGH")]
	High,
}

impl fmt::Display for AudioQuality {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::UltraLow => write!(f, "Ultra Low"),
			Self::Low => write!(f, "Low"),
			Self::Medium => write!(f, "Medium"),
			Self::High => write!(f, "High"),
		}
	}
}
