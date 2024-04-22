use gloo::storage::{LocalStorage, Storage};
use invidious::{SubsThumbsResult, SubsVideosResult, Subscription, Subscriptions};
use leptos::*;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

use super::{initial_value, load_resource, save_resource};

static SUBSCRIPTIONS_KEY: &'static str = "subscriptions";

#[derive(Copy, Clone, PartialEq)]
pub struct SubscriptionsCtx(pub RwSignal<Subscriptions>);

impl SubscriptionsCtx {
	pub fn initialise() -> Self {
		let subs = initial_value::<Subscriptions>(SUBSCRIPTIONS_KEY).unwrap_or_default();
		Self(RwSignal::new(subs))
	}

	pub async fn add_subscription(&self, id: &str, name: &str) -> Result<(), RustyTubeError> {
		self.0.update(|subs| {
			let sub = Subscription::new(id, name);
			subs.channels.push(sub);
		});
		save_resource(SUBSCRIPTIONS_KEY, self.0.get()).await
	}

	pub async fn remove_subscription(&self, id: &str) -> Result<(), RustyTubeError> {
		self.0.update(|subs| subs.channels.retain(|channel| !channel.id.eq_ignore_ascii_case(id)));
		save_resource(SUBSCRIPTIONS_KEY, self.0.get()).await
	}

	pub async fn save(&self) -> Result<(), RustyTubeError> {
		save_resource(SUBSCRIPTIONS_KEY, self.0.get()).await
	}
}

async fn fetch_subs() -> Subscriptions {
	load_resource(SUBSCRIPTIONS_KEY).await.unwrap_or_default()
}

static SUBSCRIPTIONS_VIDEOS_KEY: &'static str = "subscriptions_videos";

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
			resource: create_resource_with_initial_value(
				move || SubscriptionsVideosResourceArgs::new(subscriptions),
				move |args| fetch_subs_videos(args),
				initial_value(SUBSCRIPTIONS_KEY),
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
			resource: create_resource_with_initial_value(
				move || SubscriptionsThumbnailsResourceArgs::new(args),
				move |args| fetch_subs_thumbnails(args),
				initial_value(SUBSCRIPTIONS_THUMBNAILS_KEY),
			),
		}
	}
}

async fn fetch_subs_thumbnails(args: SubscriptionsThumbnailsResourceArgs) -> SubsThumbsResult {
	let thumbs = args.subscriptions.fetch_channel_thumbs(&args.server).await;
	save_resource(SUBSCRIPTIONS_THUMBNAILS_KEY, &thumbs.clone().unwrap()).await?;
	thumbs
}
