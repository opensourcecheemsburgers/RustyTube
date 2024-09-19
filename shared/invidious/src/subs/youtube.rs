use csv::{Reader, StringRecord};
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use super::subscriptions::{Subscription, Subscriptions};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YoutubeSubscriptions {
	pub subscriptions: Vec<YoutubeSubscription>,
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
	pub fn read_subs_from_csv(
		subs_csv_bytes: &[u8],
	) -> Result<Self, RustyTubeError> {
		let mut subs_csv = Reader::from_reader(subs_csv_bytes);
		let mut subscriptions: Vec<YoutubeSubscription> = vec![];

		let header = StringRecord::from(vec![
			"Channel ID",
			"Channel URL",
			"Channel title",
		]);

		for result in subs_csv.records() {
			let subscription: YoutubeSubscription =
				result?.deserialize(Some(&header))?;
			subscriptions.push(subscription);
		}

		Ok(Self { subscriptions })
	}
}

impl From<YoutubeSubscription> for Subscription {
	fn from(val: YoutubeSubscription) -> Self {
		let id = val.channel_id.replace("https://www.youtube.com/channel/", "");
		let name = val.channel_title;
		Self { id, name }
	}
}

impl From<YoutubeSubscriptions> for Subscriptions {
	fn from(val: YoutubeSubscriptions) -> Self {
		let mut channels: Vec<Subscription> = Vec::new();

		val.subscriptions.iter().for_each(|youtube_sub| {
			let sub: Subscription = youtube_sub.clone().into();
			channels.push(sub);
		});

		Self { channels }
	}
}
