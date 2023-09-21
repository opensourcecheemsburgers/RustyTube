use serde::{Deserialize, Serialize};
use crate::common::{CommonChannel, CommonPlaylist, CommonVideo};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SearchResult {
    #[serde(rename = "video")]
    Video(CommonVideo),
    #[serde(rename = "playlist")]
    Playlist(CommonPlaylist),
    #[serde(rename = "channel")]
    Channel(CommonChannel),
}
