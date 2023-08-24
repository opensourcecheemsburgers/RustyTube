use crate::*;
use serde::{Deserialize, Serialize};
use crate::common::{CommonChannel, CommonPlaylist, CommonVideo};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SearchItem {
    #[serde(rename = "player")]
    Video(CommonVideo),
    #[serde(rename = "playlist")]
    Playlist(CommonPlaylist),
    #[serde(rename = "channel")]
    Channel(CommonChannel),
}
