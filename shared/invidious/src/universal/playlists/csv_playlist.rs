use csv::StringRecord;
use serde::{Deserialize, Serialize};
use rustytube_error::RustyTubeError;
use utils::get_current_time;
use crate::universal::{LocalPlaylist, LocalPlaylistItem};

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

impl Into<LocalPlaylist> for CsvPlaylist {
    fn into(self) -> LocalPlaylist {
        let title = self.title;
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

impl Into<LocalPlaylistItem> for CsvPlaylistItem {
    fn into(self) -> LocalPlaylistItem {
        let id = self.id;
        LocalPlaylistItem { id }
    }
}

pub async fn read_playlist_csv(title: &str, playlist_csv_bytes: &[u8]) -> Result<LocalPlaylist, RustyTubeError> {
    let mut playlist_csv =  csv::ReaderBuilder::new().flexible(true).from_reader(playlist_csv_bytes);
    let mut playlist_items: Vec<CsvPlaylistItem> = Vec::new();

    let playlist_header = StringRecord::from(
        vec![
            "Playlist ID",
             "Channel ID",
             "Time Created",
             "Time Updated",
             "Description",
             "Visibility"
        ]
    );
    let playlist_videos_header = StringRecord::from(vec!["Video ID", "Time Added"]);

    let mut index = 0;
    for record in playlist_csv.records() {
        match index {
            0 | 1 => {},
            _ => {
                let playlist_item: CsvPlaylistItem = record?.deserialize(Some(&playlist_videos_header))?;
                playlist_items.push(playlist_item);
            }
        }
        index = index + 1;
    };
    
    Ok(CsvPlaylist { title: title.to_string(), videos: playlist_items }.into())
}