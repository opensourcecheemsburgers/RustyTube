mod csv_playlist;
mod freetube;
mod libretube;
mod local;

pub use csv_playlist::*;
pub use freetube::*;
pub use libretube::*;
pub use local::*;

use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{common::CommonImage, fetch::fetch, hidden::PlaylistItem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
	pub title: String,
	#[serde(rename = "playlistId")]
	pub id: String,
	#[serde(rename = "playlistThumbnail")]
	pub thumbnail: String,

	pub author: String,
	#[serde(rename = "authorId")]
	pub author_id: String,
	#[serde(rename = "authorThumbnails")]
	pub author_thumbnails: Vec<CommonImage>,
	pub description: String,
	#[serde(rename = "descriptionHtml")]
	pub description_html: String,

	#[serde(rename = "videoCount")]
	pub video_count: u32,
	#[serde(rename = "viewCount")]
	pub views: u64,
	pub updated: u64,
	#[serde(rename = "isListed")]
	pub listed: bool,

	pub videos: Vec<PlaylistItem>,
}

impl Playlist {
	pub async fn fetch_playlist(
		server: &str,
		id: &str,
	) -> Result<Self, RustyTubeError> {
		let url = format!("{server}/api/v1/playlists/{id}");
		let playlist_json = fetch(&url).await?;
		let playlist: Self = serde_json::from_str(&playlist_json)?;
		Ok(playlist)
	}
}
