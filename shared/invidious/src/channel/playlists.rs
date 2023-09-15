use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use crate::common::CommonPlaylist;
use crate::fetch::fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelPlaylists {
    pub playlists: Vec<CommonPlaylist>,
    pub continuation: Option<String>,
}

impl ChannelPlaylists {
    fn url(server: &str, args: &str) -> String {
        format!("{server}/api/v1/channels/playlists/{args}/")
    }

    async fn fetch_channel_playlists(server: &str, id: &str, args: Option<&str>) -> Result<Self, RustyTubeError> {
        let channel_playlists_url: String = Self::url(server, id);
        let channel_playlists_json: String = fetch(&channel_playlists_url).await?;
        let channel_playlists: Self = serde_json::from_str(&channel_playlists_json)?;
        Ok(channel_playlists)
    }
}
