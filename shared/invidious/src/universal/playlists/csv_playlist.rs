use csv::StringRecord;
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use utils::get_current_time;

use crate::universal::playlists::LocalPlaylist;
use crate::universal::playlists::LocalPlaylistItem;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CsvPlaylistItem {
	#[serde(rename = "Video ID")]
	pub id: String,
	#[serde(rename = "Time Added")]
	pub added: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CsvPlaylist {
	pub title: String,
	pub videos: Vec<CsvPlaylistItem>,
}

impl From<CsvPlaylist> for LocalPlaylist {
	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	fn from(val: CsvPlaylist) -> Self {
		let title = val.title;
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

impl From<CsvPlaylistItem> for LocalPlaylistItem {
	fn from(val: CsvPlaylistItem) -> Self {
		let id = val.id;
		Self { id }
	}
}

pub async fn read_playlist_csv(
	title: &str,
	playlist_csv_bytes: &[u8],
) -> Result<LocalPlaylist, RustyTubeError> {
	let mut playlist_csv = csv::ReaderBuilder::new()
		.flexible(true)
		.from_reader(playlist_csv_bytes);
	let mut playlist_items: Vec<CsvPlaylistItem> = Vec::new();

	let _playlist_header = StringRecord::from(vec![
		"Playlist ID",
		"Channel ID",
		"Time Created",
		"Time Updated",
		"Description",
		"Visibility",
	]);
	let playlist_videos_header =
		StringRecord::from(vec!["Video ID", "Time Added"]);

	let mut index = 0;
	for (mut index, record) in playlist_csv.records().enumerate() {
		match index {
			0 | 1 => {}
			_ => {
				let playlist_item: CsvPlaylistItem =
					record?.deserialize(Some(&playlist_videos_header))?;
				playlist_items.push(playlist_item);
			}
		}
		index += 1;
	}

	Ok(CsvPlaylist { title: title.to_string(), videos: playlist_items }.into())
}
