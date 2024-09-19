use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use utils::get_current_time;

use crate::universal::{
	LocalPlaylist, LocalPlaylistItem, LOCAL_PLAYLIST_PREFIX,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibretubePlaylists {
	pub format: String,
	pub version: u32,
	pub playlists: Vec<LibretubePlaylist>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibretubePlaylist {
	pub name: String,
	#[serde(rename = "type")]
	pub playlist_type: String,
	pub visibility: String,
	pub videos: Vec<String>,
}

impl From<LibretubePlaylists> for Vec<LocalPlaylist> {
	fn from(val: LibretubePlaylists) -> Self {
		let mut local_playlists = vec![];
		val.playlists.into_iter().for_each(|playlist| {
			local_playlists.push(playlist.into());
		});
		local_playlists
	}
}

impl From<LibretubePlaylist> for LocalPlaylist {
	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	fn from(val: LibretubePlaylist) -> Self {
		let title = format!("{}{}", LOCAL_PLAYLIST_PREFIX, val.name);
		let video_count = val.videos.len() as u32;
		let updated = get_current_time().unwrap_or_default() as u64;
		let created = updated;

		let mut videos: Vec<LocalPlaylistItem> = Vec::new();
		val.videos
			.into_iter()
			.for_each(|video| videos.push(LocalPlaylistItem { id: video }));

		Self { title, video_count, updated, created, videos }
	}
}

pub async fn read_libretube_playlists(
	playlist_json: &str,
) -> Result<Vec<LocalPlaylist>, RustyTubeError> {
	let playlists: LibretubePlaylists = serde_json::from_str(playlist_json)?;
	Ok(playlists.into())
}
