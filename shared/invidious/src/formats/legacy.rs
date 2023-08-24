use serde::{Deserialize, Serialize};
use crate::formats::items::{Container, Resolution, QualityLabel};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LegacyFormat {
    pub url: String,
    pub itag: String,
    pub r#type: String,
    pub quality: String,
    #[serde(default)]
    pub container: Container,
    #[serde(default)]
    pub encoding: String,
    #[serde(rename = "qualityLabel")]
    pub quality_label: QualityLabel,
    pub resolution: Resolution,
    pub size: String,
}