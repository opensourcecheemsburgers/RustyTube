use serde::{Deserialize, Serialize};
use rustytube_error::RustyTubeError;

use super::subscriptions::*;

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
