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

    pub async fn fetch_channel_videos(server: &str, id: &str) -> Result<Vec<CommonVideo>, RustyTubeError> {
        let url = format!("{}/api/v1/channels/{}/videos/", server, id);
        let channel_videos_json: String = fetch(&url).await?;
        let channel_videos: Self = serde_json::from_str(&channel_videos_json)?;
        Ok(channel_videos.videos)
    }
}
