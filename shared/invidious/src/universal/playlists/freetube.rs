use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use utils::get_current_time;

use crate::{LocalPlaylist, LocalPlaylistItem, LOCAL_PLAYLIST_PREFIX};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FreetubePlaylists {
	pub playlists: Vec<FreetubePlaylist>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FreetubePlaylist {
	#[serde(rename = "playlistName")]
	pub playlist_name: String,
	pub videos: Vec<FreetubePlaylistItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FreetubePlaylistItem {
	#[serde(rename = "videoId")]
	pub id: String,
	pub title: String,
	pub author: String,
	#[serde(rename = "authorId")]
	pub author_id: String,
}

pub async fn read_freetube_playlists(
	playlist_json: &str,
) -> Result<Vec<LocalPlaylist>, RustyTubeError> {
	// let playlists: FreetubePlaylists = serde_json::from_str(playlist_json)?;
	// Ok(playlists.into()
	let playlists: Vec<FreetubePlaylist> = serde_json::from_str(playlist_json)?;
	let mut local_playlists = Vec::new();
	playlists
		.into_iter()
		.for_each(|playlist| local_playlists.push(playlist.into()));
	Ok(local_playlists)
}

impl From<FreetubePlaylists> for Vec<LocalPlaylist> {
	fn from(val: FreetubePlaylists) -> Self {
		let mut local_playlists = vec![];

		val.playlists.into_iter().for_each(|playlist| {
			local_playlists.push(playlist.into());
		});

		local_playlists
	}
}

impl From<FreetubePlaylist> for LocalPlaylist {
	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	fn from(val: FreetubePlaylist) -> Self {
		let title = format!("{}{}", LOCAL_PLAYLIST_PREFIX, &val.playlist_name);
		let video_count = val.videos.len() as u32;
		let updated = get_current_time().unwrap_or_default() as u64;
		let created = updated;
		let mut videos = Vec::new();

		val.videos.into_iter().for_each(|video| {
			videos.push(video.into());
		});

		Self { title, video_count, updated, created, videos }
	}
}

impl From<FreetubePlaylistItem> for LocalPlaylistItem {
	fn from(val: FreetubePlaylistItem) -> Self {
		let id = val.id;
		Self { id }
	}
}
