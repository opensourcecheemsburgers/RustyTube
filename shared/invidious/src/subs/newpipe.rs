use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use super::subscriptions::{Subscription, Subscriptions};

impl From<NewpipeSubscription> for Subscription {
	fn from(val: NewpipeSubscription) -> Self {
		let index =
			val.url.rfind("UC").expect("Channel id should have UC at start");
		let mut id = val.url.clone();
		id.replace_range(0..index, "");
		let name = val.name;

		Self { id, name }
	}
}

impl From<NewpipeSubscriptions> for Subscriptions {
	fn from(val: NewpipeSubscriptions) -> Self {
		let mut channels: Vec<Subscription> = Vec::new();

		val.subscriptions.iter().for_each(|newpipe_sub| {
			let sub: Subscription = newpipe_sub.clone().into();
			channels.push(sub);
		});

		Self { channels }
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
	pub fn read_subs_from_file(
		subs_json: &str,
	) -> Result<Self, RustyTubeError> {
		let subbed_channels: Self = serde_json::from_str(subs_json)?;
		Ok(subbed_channels)
	}

	pub fn to_json_string(&self) -> Result<String, RustyTubeError> {
		Ok(serde_json::to_string_pretty(self)?)
	}
}
