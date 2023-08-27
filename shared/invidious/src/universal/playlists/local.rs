use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use gloo::file::Blob;
use gloo::file::futures::{read_as_bytes, read_as_text};

use utils::{save_to_browser_storage, load_all_from_browser_storage, get_current_time_rfc};
use crate::universal::playlists::freetube::read_freetube_playlists;
use crate::universal::playlists::libretube::read_libretube_playlists;
use crate::universal::read_playlist_csv;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalPlaylistItem {
    pub id: String,
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
                            Err(_) => Err(RustyTubeError::parse_playlist_fail())
                        }
                    }
                }
            false => {
                match read_freetube(&file).await {
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
                            Err(_) => Err(RustyTubeError::parse_playlist_fail())
                        }
                    }
                }
            }
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