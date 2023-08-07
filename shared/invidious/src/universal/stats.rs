use serde::{Deserialize, Serialize};
use crate::hidden::{Metadata, Software, Usage};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub version: String,
    pub software: Software,
    #[serde(rename = "openRegistrations")]
    pub registrations: bool,
    pub usage: Usage,
    pub metadata: Metadata,
}

impl Stats {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/stats/{}", server, args)
    }
}
