use serde::{Deserialize, Serialize};

use crate::common::{CommonChannel, CommonPlaylist, CommonVideo};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum SearchResult {
	#[serde(rename = "video")]
	Video(CommonVideo),
	#[serde(rename = "playlist")]
	Playlist(CommonPlaylist),
	#[serde(rename = "channel")]
	Channel(CommonChannel),
}
