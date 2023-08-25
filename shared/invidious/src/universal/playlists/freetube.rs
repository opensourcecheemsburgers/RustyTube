use serde::{Deserialize, Serialize};
use rustytube_error::RustyTubeError;
use utils::{get_current_time};
use crate::universal::{LOCAL_PLAYLIST_PREFIX, LocalPlaylist, LocalPlaylistItem};

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

pub async fn read_freetube_playlists(playlist_json: &str) -> Result<Vec<LocalPlaylist>, RustyTubeError> {
    // let playlists: FreetubePlaylists = serde_json::from_str(playlist_json)?;
    // Ok(playlists.into()
    let playlists: Vec<FreetubePlaylist> = serde_json::from_str(playlist_json)?;
    let mut local_playlists = Vec::new();
    playlists.into_iter().for_each(|playlist| {
        local_playlists.push(playlist.into())
    });
    Ok(local_playlists)
}

impl Into<Vec<LocalPlaylist>> for FreetubePlaylists {
    fn into(self) -> Vec<LocalPlaylist> {
        let mut local_playlists: Vec<LocalPlaylist> = Vec::new();

        self.playlists.into_iter().for_each(|playlist| {
            local_playlists.push(playlist.into());
        });

        local_playlists
    }
}


impl Into<LocalPlaylist> for FreetubePlaylist {
    fn into(self) -> LocalPlaylist {
        let title = format!("{}{}", LOCAL_PLAYLIST_PREFIX, &self.playlist_name);
        let video_count = self.videos.len() as u32;
        let updated = get_current_time();
        let created = updated;
        let mut videos = Vec::new();

        self.videos.into_iter().for_each(|video| {
            videos.push(video.into());
        });

        LocalPlaylist { title, video_count, updated, created, videos }
    }
}

impl Into<LocalPlaylistItem> for FreetubePlaylistItem {
    fn into(self) -> LocalPlaylistItem {
        let id = self.id;
        LocalPlaylistItem { id }
    }
}