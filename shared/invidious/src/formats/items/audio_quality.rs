use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
			AudioQuality::UltraLow => write!(f, "Ultra Low"),
			AudioQuality::Low => write!(f, "Low"),
			AudioQuality::Medium => write!(f, "Medium"),
			AudioQuality::High => write!(f, "High"),
		}
	}
}