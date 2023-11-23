use crate::{Feed, YoutubeSubscriptions, NewpipeSubscriptions, Channel, ChannelVideos, ChannelThumb, YoutubeSubscription, NewpipeSubscription};
use gloo::storage::{LocalStorage, Storage};
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use futures::future::join_all;
use gloo::file::Blob;
use gloo::file::futures::{read_as_bytes, read_as_text};
use utils::save_to_browser_storage;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Subscriptions {
    pub channels: Vec<Subscription>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Subscription {
    pub id: String,
    pub name: String
}

impl Subscription {
    pub fn new(id: &str, name: &str) -> Self {
        Self { id: id.to_owned(), name: name.to_owned() }
    }
}

pub const SUBS_KEY: &'static str = "subscriptions";

pub type SubscriptionVideos = Result<ChannelVideos, RustyTubeError>;
pub type SubscriptionsVideos = Vec<SubscriptionVideos>;
pub type SubscriptionsFetch = Result<SubscriptionsVideos, RustyTubeError>;

impl Subscriptions {
    pub async fn read_subs(blob: Blob) -> Result<Self, RustyTubeError> {
        match blob.raw_mime_type().eq("text/csv") {
            true => match read_youtube(&blob).await {
                Ok(subs) => Ok(subs),
                Err(_) => match read_newpipe(&blob).await {
                    Ok(subs) => Ok(subs),
                    Err(err) => Err(err),
                }
            },
            false => match read_newpipe(&blob).await {
                Ok(subs) => Ok(subs),
                Err(_) => match read_youtube(&blob).await {
                    Ok(subs) => Ok(subs),
                    Err(err) => Err(err),
                }
            }
        }
    }

    pub fn to_ron_string(&self) -> Result<String, RustyTubeError> {
        Ok(ron::to_string(&self)?)
    }

    pub async fn save(&self) -> Result<(), RustyTubeError> {
        let subs_ron_str = ron::to_string(&self)?;
        save_to_browser_storage(SUBS_KEY, &subs_ron_str)?;
        Ok(())
    }

    pub fn load() -> Result<Self, RustyTubeError> {
        let subs_ron_str: String = LocalStorage::get(SUBS_KEY)?;
        let subs: Subscriptions = ron::from_str(&subs_ron_str)?;
        Ok(subs)
    }

    pub async fn fetch_videos(&self, server: &str, rss: bool) -> SubscriptionsFetch {
        let mut futures = Vec::new();

        for channel in self.channels.clone() {
            let id = channel.id.clone();
            let future = async move {
                match rss {
                    true => Feed::fetch_videos_from_feed(server, &id).await,
                    false => Channel::fetch_channel_videos(server, &id, None).await,
                }
            };
            futures.push(future)
        }
        let subs_videos = join_all(futures).await;

        Ok(subs_videos)
    }

    pub async fn fetch_channels(&self, server: &str) -> Result<Vec<Result<Channel, RustyTubeError>>, RustyTubeError> {
        let mut futures = Vec::new();

        for channel in self.channels.clone() {
            let id = channel.id.clone();
            let future = async move { Channel::fetch_channel(server, &id).await};
            futures.push(future)
        }
        let channels = join_all(futures).await;
        Ok(channels)
    }

    pub async fn fetch_channel_thumbs(&self, server: &str) -> Result<Vec<Result<ChannelThumb, RustyTubeError>>, RustyTubeError> {
        let mut futures = Vec::new();

        for channel in self.channels.clone() {
            let id = channel.id.clone();
            let future = async move { Channel::fetch_channel_thumb(server, &id).await};
            futures.push(future)
        }
        let thumbs = join_all(futures).await;
        Ok(thumbs)
    }
}

async fn read_youtube(file: &Blob) -> Result<Subscriptions, RustyTubeError> {
    let bytes = read_as_bytes(&file).await?;
    let slice = bytes.as_slice();
    let yt_subs = YoutubeSubscriptions::read_subs_from_csv(&slice)?;
    Ok(yt_subs.into())
}

async fn read_newpipe(file: &Blob) -> Result<Subscriptions, RustyTubeError> {
    let json_str = read_as_text(&file).await?;
    let newpipe_subs = NewpipeSubscriptions::read_subs_from_file(&json_str)?;
    Ok(newpipe_subs.into())
}

impl Into<NewpipeSubscriptions> for Subscriptions {
    fn into(self) -> NewpipeSubscriptions {
        let subscriptions = self.channels.into_iter().map(|yt_sub| yt_sub.into()).collect::<Vec<NewpipeSubscription>>();
        
        NewpipeSubscriptions { app_version: "0.0.0".to_string(), app_version_int: 0, subscriptions  }
    }  
}

impl Into<NewpipeSubscription> for Subscription {
    fn into(self) -> NewpipeSubscription {
        let service_id = 0;
        let name = self.name;
        let url = format!("https:://youtube.com/channel/{}", self.id);
        
        NewpipeSubscription { service_id, name, url }
    }
}

impl Into<YoutubeSubscriptions> for Subscriptions {
    fn into(self) -> YoutubeSubscriptions {
        let subscriptions = self.channels.into_iter().map(|yt_sub| yt_sub.into()).collect::<Vec<YoutubeSubscription>>();
        
        YoutubeSubscriptions { subscriptions }
    }  
}

impl Into<YoutubeSubscription> for Subscription {
    fn into(self) -> YoutubeSubscription {
        let channel_id = self.id.clone();
        let channel_url = format!("https:://youtube.com/channel/{}", self.id);
        let channel_title = self.name;
    
        YoutubeSubscription { channel_id, channel_url, channel_title }
    }
}
