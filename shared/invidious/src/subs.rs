use std::io::Bytes;
use csv::{ByteRecord, Reader, ReaderBuilder, StringRecord};
use serde::{Deserialize, Serialize};
use crate::channel::Channel;
use crate::error::RustyTubeError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscriptions {
    pub channels: Vec<Subscription>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscription {
    pub id: String,
    pub name: String,
}

impl Into<Subscription> for NewpipeSubscription {
    fn into(self) -> Subscription {
        let id = self.url.replace("https://www.youtube.com/channel/", "");
        let name = self.name.to_string();
        Subscription { id, name }
    }
}

impl Into<Subscriptions> for NewpipeSubscriptions {
    fn into(self) -> Subscriptions {
        let mut channels: Vec<Subscription> = Vec::new();

        self.subscriptions.iter().for_each(|newpipe_sub| {
            let sub: Subscription = newpipe_sub.clone().into();
            channels.push(sub)
        });

        Subscriptions { channels }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewpipeSubscription {
    pub service_id: u32,
    pub name: String,
    pub url: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewpipeSubscriptions {
    pub app_version: String,
    pub app_version_int: u32,
    pub subscriptions: Vec<NewpipeSubscription>
}

impl NewpipeSubscriptions {
    pub fn read_subs_from_file(subs_json: &str) -> Result<Self, RustyTubeError> {
        let subbed_channels: Self = serde_json::from_str(subs_json)?;
        Ok(subbed_channels)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YoutubeSubscriptions {
    pub subscriptions: Vec<YoutubeSubscription>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YoutubeSubscription {
    #[serde(rename = "Channel ID")]
    pub channel_id: String,
    #[serde(rename = "Channel URL")]
    pub channel_url: String,
    #[serde(rename = "Channel title")]
    pub channel_title: String,
}

impl YoutubeSubscriptions {
    pub fn read_subs_from_csv(subs_csv_bytes: &[u8]) -> Result<Self, RustyTubeError> {
        let mut subs_csv = Reader::from_reader(subs_csv_bytes);
        let mut subscriptions: Vec<YoutubeSubscription> = vec![];

        let header = StringRecord::from(vec![
            "Channel ID", "Channel URL", "Channel title",
        ]);

        for result in subs_csv.records() {
            let subscription: YoutubeSubscription = result?.deserialize(Some(&header))?;
            subscriptions.push(subscription);
        }

        Ok(YoutubeSubscriptions { subscriptions })
    }
}

impl Into<Subscription> for YoutubeSubscription {
    fn into(self) -> Subscription {
        let id = self.channel_id.replace("https://www.youtube.com/channel/", "");
        let name = self.channel_title.to_string();
        Subscription { id, name }
    }
}

impl Into<Subscriptions> for YoutubeSubscriptions {
    fn into(self) -> Subscriptions {
        let mut channels: Vec<Subscription> = Vec::new();

        self.subscriptions.iter().for_each(|youtube_sub| {
            let sub: Subscription = youtube_sub.clone().into();
            channels.push(sub)
        });

        Subscriptions { channels }
    }
}