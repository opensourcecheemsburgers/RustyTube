use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use super::subscriptions::*;

impl Into<Subscription> for NewpipeSubscription {
    fn into(self) -> Subscription {
        let index = self
            .url
            .rfind("UC")
            .expect("Channel id should have UC at start");
        let mut id = self.url.clone();
        id.replace_range(0..index, "");
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
    pub name: String,
    pub service_id: u32,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewpipeSubscriptions {
    pub app_version: String,
    pub app_version_int: u32,
    pub subscriptions: Vec<NewpipeSubscription>,
}

impl NewpipeSubscriptions {
    pub fn read_subs_from_file(subs_json: &str) -> Result<Self, RustyTubeError> {
        let subbed_channels: Self = serde_json::from_str(subs_json)?;
        Ok(subbed_channels)
    }

    pub fn to_json_string(&self) -> Result<String, RustyTubeError> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}
