use crate::{CommonVideo, ChannelVideos, Feed};
use gloo::storage::{LocalStorage, Storage};
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use futures::future::{join_all};
use utils::save_to_browser_storage;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Subscriptions {
    pub channels: Vec<Subscription>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Subscription {
    pub id: String,
    pub name: String,
}

pub const SUBS_KEY: &'static str = "subscriptions";

pub type SubscriptionVideos = Result<Vec<CommonVideo>, RustyTubeError>;
pub type SubscriptionsVideos = Vec<SubscriptionVideos>;
pub type SubscriptionsFetch = Result<SubscriptionsVideos, RustyTubeError>;

impl Subscriptions {
    pub async fn from_ron_str(ron_str: &str) -> Result<Self, RustyTubeError> {
        Ok(ron::from_str(&ron_str)?)
    }

    pub async fn save(&self) -> Result<(), RustyTubeError> {
        let subs_json = serde_json::to_string(&self)?;
        save_to_browser_storage(SUBS_KEY, &subs_json)?;
        Ok(())
    }

    pub fn load() -> Result<Self, RustyTubeError> {
        let subs: Subscriptions = LocalStorage::get(SUBS_KEY)?;
        Ok(subs)
    }

    pub async fn fetch_subs(&self, server: &str, rss: bool) -> SubscriptionsFetch {
        let mut futures = Vec::new();

        for channel in self.channels.clone() {
            let id = channel.id.clone();
            let future = async move {
                match rss {
                    true => Feed::fetch_videos_from_feed(server, &id).await,
                    false => ChannelVideos::fetch_channel_videos(server, &id).await,
                }
            };
            futures.push(future)
        }
        let subs_videos = join_all(futures).await;
        Ok(subs_videos)
    }
}
