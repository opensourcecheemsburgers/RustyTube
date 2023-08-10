mod local;
mod csv_playlist;
mod freetube;
mod libretube;

pub use local::*;
pub use csv_playlist::*;
pub use libretube::*;
pub use freetube::*;

use serde::{Deserialize, Serialize};
use crate::common::CommonImage;
use rustytube_error::RustyTubeError;
use crate::fetch::fetch;
use crate::hidden::PlaylistItem;

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
    fn url(server: &str, args: &str) -> String {
        format!("{}/api/v1/playlists/{}", server, args)
    }

    pub async fn fetch_playlist(server: &str, id: &str, args: Option<&str>) -> Result<Self, RustyTubeError> {
        let url = Self::url(server, id);
        let playlist_json = fetch(&url).await?;
        let playlist: Self = serde_json::from_str(&playlist_json)?;
        Ok(playlist)
    }
}