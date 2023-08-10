use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use crate::common::CommonVideo;
use crate::fetch::fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelVideos {
    pub videos: Vec<CommonVideo>,
}

impl ChannelVideos {
    fn url(server: &str, args: &str) -> String {
        format!("{server}/api/v1/channels/videos/{args}")
    }

    async fn fetch_channel_videos(server: &str, args: &str) -> Result<Self, RustyTubeError> {
        let channel_videos_url: String = Self::url(server, args);
        let channel_videos_json: String = fetch(&channel_videos_url).await?;
        let channel_videos: Self = serde_json::from_str(&channel_videos_json)?;
        Ok(channel_videos)
    }
}
