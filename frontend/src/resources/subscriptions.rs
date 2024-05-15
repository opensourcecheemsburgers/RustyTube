use gloo::storage::{LocalStorage, Storage};
use invidious::{SubsThumbsResult, SubsVideosResult, Subscription, Subscriptions};
use leptos::*;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

use super::save_resource;

static SUBSCRIPTIONS_KEY: &'static str = "subscriptions";

#[derive(Copy, Clone, PartialEq)]
pub struct SubscriptionsCtx(pub RwSignal<Subscriptions>);

impl SubscriptionsCtx {
	pub fn initialise() -> Self {
		Self(RwSignal::new(get_subs(SUBSCRIPTIONS_KEY).unwrap_or_default()))
	}

	pub async fn add_subscription(&self, id: &str, name: &str) -> Result<(), RustyTubeError> {
		self.0.update(|subs| {
			let sub = Subscription::new(id, name);
			if subs.channels.iter().find(|sub| sub.id.eq_ignore_ascii_case(id)).is_none() {
				subs.channels.push(sub);
				subs.channels.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
				subs.channels.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));
				save_subs(subs);
			}
		});
		Ok(())
	}

	pub async fn remove_subscription(&self, id: &str) -> Result<(), RustyTubeError> {
		self.0.update(|subs| {
			subs.channels.retain(|channel| !channel.id.eq_ignore_ascii_case(id));
			subs.channels.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
			subs.channels.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));
			save_subs(subs);
		});
		Ok(())
	}
}

pub fn get_subs(key: &'static str) -> Result<Subscriptions, RustyTubeError> {
	Ok(LocalStorage::get::<Subscriptions>(key)?)
}

pub fn save_subs(subs: &mut Subscriptions) -> Result<(), RustyTubeError> {
	LocalStorage::set(SUBSCRIPTIONS_KEY, subs);
	Ok(())
}

#[derive(Clone, PartialEq)]
pub struct SubscriptionsVideosResourceArgs {
	server: String,
	locale: RustyTubeLocale,
	subscriptions: Subscriptions,
}

impl SubscriptionsVideosResourceArgs {
	pub fn new(subscriptions: SubscriptionsCtx) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			subscriptions: subscriptions.0.get(),
		}
	}
}

#[derive(Copy, Clone)]
pub struct SubscriptionsVideosResource {
	pub resource: Resource<SubscriptionsVideosResourceArgs, SubsVideosResult>,
}

impl SubscriptionsVideosResource {
	pub fn initialise(subscriptions: SubscriptionsCtx) -> Self {
		SubscriptionsVideosResource {
			resource: Resource::local(
				move || SubscriptionsVideosResourceArgs::new(subscriptions),
				move |args| fetch_subs_videos(args),
			),
		}
	}
}

async fn fetch_subs_videos(args: SubscriptionsVideosResourceArgs) -> SubsVideosResult {
	let videos = args
		.subscriptions
		.fetch_videos(&args.server, false, &args.locale.to_invidious_lang())
		.await;
	// save_resource(SUBSCRIPTIONS_VIDEOS_KEY, &videos).await?;
	videos
}

static SUBSCRIPTIONS_THUMBNAILS_KEY: &'static str = "subscriptions_thumbs";

#[derive(Clone, PartialEq)]
pub struct SubscriptionsThumbnailsResourceArgs {
	server: String,
	subscriptions: Subscriptions,
}

impl SubscriptionsThumbnailsResourceArgs {
	pub fn new(subscriptions: SubscriptionsCtx) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			subscriptions: subscriptions.0.get(),
		}
	}
}

#[derive(Copy, Clone)]
pub struct SubscriptionsThumbnailsResource {
	pub resource: Resource<SubscriptionsThumbnailsResourceArgs, SubsThumbsResult>,
}

impl SubscriptionsThumbnailsResource {
	pub fn initialise(args: SubscriptionsCtx) -> Self {
		SubscriptionsThumbnailsResource {
			resource: Resource::local(
				move || SubscriptionsThumbnailsResourceArgs::new(args),
				move |args| fetch_subs_thumbnails(args),
			),
		}
	}
}

async fn fetch_subs_thumbnails(args: SubscriptionsThumbnailsResourceArgs) -> SubsThumbsResult {
	let thumbs = args.subscriptions.fetch_channel_thumbs(&args.server).await;
	thumbs
}
