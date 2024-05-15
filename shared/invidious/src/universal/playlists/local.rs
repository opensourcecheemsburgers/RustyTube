use std::str::FromStr;

use futures::future::join_all;
use gloo::file::{
	futures::{read_as_bytes, read_as_text},
	Blob,
};
use leptos::*;
use leptos_router::*;
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use utils::{get_current_time_rfc, load_all_from_browser_storage, save_to_browser_storage};

use crate::{
	fetch,
	universal::{
		playlists::{freetube::read_freetube_playlists, libretube::read_libretube_playlists},
		read_playlist_csv,
	},
	CommonThumbnail, Video,
};

pub const LOCAL_PLAYLIST_PREFIX: &'static str = "rt_playlist_";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalPlaylist {
	pub title: String,
	#[serde(rename = "videoCount")]
	pub video_count: u32,
	#[serde(rename = "viewCount")]
	pub updated: u64,
	pub created: u64,
	pub videos: Vec<LocalPlaylistItem>,
}

impl PartialEq for LocalPlaylist {
	fn eq(&self, other: &Self) -> bool {
		self.created == other.created
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalPlaylistItem {
	pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalPlaylistVideo {
	pub title: String,
	#[serde(rename = "videoId")]
	pub id: String,
	pub author: String,
	#[serde(rename = "authorId")]
	pub author_id: String,
	#[serde(rename = "videoThumbnails")]
	pub thumbnails: Vec<CommonThumbnail>,
	#[serde(rename = "viewCount")]
	pub views: u64,
}

impl LocalPlaylist {
	pub async fn create(title: &str) -> Result<Self, RustyTubeError> {
		let title = title.to_string();
		let video_count = 0;
		let updated = 0;
		let videos: Vec<LocalPlaylistItem> = Vec::new();
		let created = utils::get_current_time();

		// let performance = gloo::utils::window().performance();

		Ok(Self { title, video_count, updated, videos, created })
	}

	pub async fn save(&self) -> Result<(), RustyTubeError> {
		let playlist_json = serde_json::to_string_pretty(&self)?;
		let key = format!("{}{}", LOCAL_PLAYLIST_PREFIX, &self.title);

		save_to_browser_storage(&key, &playlist_json)?;
		Ok(())
	}

	pub async fn load_local_playlists() -> Result<Vec<Self>, RustyTubeError> {
		let mut playlists_vec: Vec<LocalPlaylist> = Vec::new();

		let storage_map = load_all_from_browser_storage()?;
		storage_map.iter().for_each(|item| {
			if item.0.starts_with(LOCAL_PLAYLIST_PREFIX) {
				if let Ok(playlist) = serde_json::from_value(item.1.to_owned()) {
					playlists_vec.push(playlist);
				}
			};
		});

		Ok(playlists_vec)
	}

	pub async fn save_playlists(playlists: &Vec<Self>) -> Result<(), RustyTubeError> {
		for playlist in playlists {
			playlist.save().await?;
		}
		Ok(())
	}

	pub async fn fetch_first_playlist_video(&self, server: &str) -> Result<Video, RustyTubeError> {
		let video_url = format!(
			"{}/api/v1/videos/{}/",
			server,
			self.videos.first().cloned().expect("Playlist should not be empty.").id
		);
		let video_json = fetch(&video_url).await?;
		Ok(serde_json::from_str::<Video>(&video_json)?)
	}

	pub async fn fetch_playlist_videos(&self, server: &str) -> Vec<Result<Video, RustyTubeError>> {
		let mut videos = vec![];

		for video in self.videos.clone() {
			let future = async move {
				let video_url = format!("{}/api/v1/videos/{}/", server, video.id);
				let video_json = fetch(&video_url).await?;
				Ok(serde_json::from_str::<Video>(&video_json)?)
			};
			videos.push(future)
		}

		join_all(videos).await
	}

	pub async fn read_playlists(file: Blob) -> Result<Vec<Self>, RustyTubeError> {
		let mime = file.raw_mime_type();

		let mut local_playlists: Vec<LocalPlaylist> = Vec::new();

		match mime.eq_ignore_ascii_case("text/csv") {
			true => match read_csv(&file).await {
				Ok(playlist) => {
					local_playlists.push(playlist);
					LocalPlaylist::save_playlists(&local_playlists).await?;
					Ok(local_playlists)
				}
				Err(_) => match read_freetube(&file).await {
					Ok(mut playlists) => {
						local_playlists.append(&mut playlists);
						LocalPlaylist::save_playlists(&local_playlists).await?;
						Ok(local_playlists)
					}
					Err(_) => match read_libretube(&file).await {
						Ok(mut playlists) => {
							local_playlists.append(&mut playlists);
							LocalPlaylist::save_playlists(&local_playlists).await?;
							Ok(local_playlists)
						}
						Err(_) => Err(RustyTubeError::PlaylistParse),
					},
				},
			},
			false => match read_freetube(&file).await {
				Ok(mut playlists) => {
					local_playlists.append(&mut playlists);
					LocalPlaylist::save_playlists(&local_playlists).await?;
					Ok(local_playlists)
				}
				Err(_) => match read_libretube(&file).await {
					Ok(mut playlists) => {
						local_playlists.append(&mut playlists);
						LocalPlaylist::save_playlists(&local_playlists).await?;
						Ok(local_playlists)
					}
					Err(_) => match read_csv(&file).await {
						Ok(playlist) => {
							local_playlists.push(playlist);
							LocalPlaylist::save_playlists(&local_playlists).await?;
							Ok(local_playlists)
						}
						Err(_) => Err(RustyTubeError::PlaylistParse),
					},
				},
			},
		}
	}
}

async fn read_csv(file: &Blob) -> Result<LocalPlaylist, RustyTubeError> {
	let bytes = read_as_bytes(&file).await?;
	let slice = bytes.as_slice();
	let playlist = read_playlist_csv(&get_current_time_rfc(), slice).await?;
	Ok(playlist)
}

async fn read_libretube(file: &Blob) -> Result<Vec<LocalPlaylist>, RustyTubeError> {
	let mut local_playlists: Vec<LocalPlaylist> = Vec::new();
	let json_string = read_as_text(&file).await?;
	let mut playlists = read_libretube_playlists(&json_string).await?;
	local_playlists.append(&mut playlists);
	Ok(local_playlists)
}

async fn read_freetube(file: &Blob) -> Result<Vec<LocalPlaylist>, RustyTubeError> {
	let mut local_playlists: Vec<LocalPlaylist> = Vec::new();
	let json_string = read_as_text(&file).await?;
	let mut playlists = read_freetube_playlists(&json_string).await?;
	local_playlists.append(&mut playlists);
	Ok(local_playlists)
}
