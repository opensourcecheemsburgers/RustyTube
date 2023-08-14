use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdaptiveFormat {
    #[serde(default)]
    pub index: String,
    #[serde(default)]
    pub bitrate: String,
    #[serde(default)]
    pub init: String,
    pub url: String,
    pub itag: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub clen: String,
    #[serde(default)]
    pub lmt: String,
    #[serde(default)]
    #[serde(rename = "protectionType")]
    pub projection_type: String,
    #[serde(default)]
    pub fps: u8,
    #[serde(default)]
    pub container: String,
    #[serde(default)]
    pub encoding: String,
    #[serde(default)]
    #[serde(rename = "qualityLabel")]
    pub quality: String,
    #[serde(default)]
    pub resolution: String,
    #[serde(default)]
    #[serde(rename = "qualityLabel")]
    pub quality_label: String,
    #[serde(default)]
    #[serde(rename = "audioQuality")]
    pub audio_quality: String,
    #[serde(default)]
    #[serde(rename = "audioSampleRate")]
    pub audio_sample_rate: usize,
    #[serde(default)]
    #[serde(rename = "audioChannels")]
    pub audio_channels: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum VideoFormat {
    #[serde(rename = "mp4")]
    MP4,
    #[serde(rename = "webm")]
    WEBM,
    #[serde(rename = "3gp")]
    _3GP,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AudioFormat {
    #[serde(rename = "m4a")]
    M4A,
    #[serde(rename = "webm")]
    WEBM,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Quality {
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
    _4320p
}