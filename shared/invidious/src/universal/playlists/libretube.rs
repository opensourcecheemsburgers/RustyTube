use serde::{Deserialize, Serialize};
use rustytube_error::RustyTubeError;
use utils::get_current_time;
use crate::universal::{LOCAL_PLAYLIST_PREFIX, LocalPlaylist, LocalPlaylistItem};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibretubePlaylists {
    pub format: String,
    pub version: u32,
    pub playlists: Vec<LibretubePlaylist>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibretubePlaylist {
    pub name: String,
    #[serde(rename = "type")]
    pub playlist_type: String,
    pub visibility: String,
    pub videos: Vec<String>
}

impl Into<Vec<LocalPlaylist>> for LibretubePlaylists {
    fn into(self) -> Vec<LocalPlaylist> {
        let mut local_playlists: Vec<LocalPlaylist> = Vec::new();
        self.playlists.into_iter().for_each(|playlist| {
            local_playlists.push(playlist.into());
        });
        local_playlists
    }
}

impl Into<LocalPlaylist> for LibretubePlaylist {
    fn into(self) -> LocalPlaylist {
        let title = format!("{}{}", LOCAL_PLAYLIST_PREFIX, self.name);
        let video_count = self.videos.len() as u32;
        let updated = get_current_time();
        let created = updated;
        
        let mut videos: Vec<LocalPlaylistItem> = Vec::new();
        self.videos.into_iter().for_each(|video| {
            videos.push(LocalPlaylistItem { id: video })
        });
        
        LocalPlaylist { title, video_count, updated, created, videos }
    }
}

pub async fn read_libretube_playlists(playlist_json: &str) -> Result<Vec<LocalPlaylist>, RustyTubeError> {
    let playlists: LibretubePlaylists = serde_json::from_str(playlist_json)?;
    Ok(playlists.into())
}

