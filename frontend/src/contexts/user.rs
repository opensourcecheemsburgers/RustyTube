use gloo::storage::LocalStorage;
use invidious::{Channel, CommonVideo, fetch_instance_info, Instances, Subscriptions, SubscriptionsFetch, Subscription};
use leptos::*;
use wasm_bindgen_futures::spawn_local;
use rustytube_error::RustyTubeError;
use crate::contexts::ServerCtx;

#[derive(Copy, Clone)]
pub struct SubscriptionsCtx(pub RwSignal<Subscriptions>);

impl SubscriptionsCtx {
	pub async fn add_subscription(&self, id: &str, name: &str) -> Result<(), RustyTubeError> {
		self.0.update(|subs| {
				let sub = Subscription::new(id, name);
				subs.channels.push(sub);
		});
		self.0.get().save().await?;
		Ok(())
	}
	pub async fn remove_subscription(&self, id: &str) -> Result<(), RustyTubeError> {
		self.0.update(|subs| subs.channels.retain(|channel| !channel.id.eq_ignore_ascii_case(id)));
		self.0.get().save().await?;
		Ok(())
	}
}

#[derive(Copy, Clone)]
pub struct ChannelsCtx(pub Resource<(String, Subscriptions), Vec<Result<Channel, RustyTubeError>>>);

#[derive(Copy, Clone)]
pub struct SubsVideosCtx(pub Resource<(String, Subscriptions), SubscriptionsFetch>);

#[derive(Copy, Clone)]
pub struct InstancesCtx(pub Resource<(), Result<Instances, RustyTubeError>>);

pub fn provide_user_contexts() {
	let subscriptions = Subscriptions::load().unwrap_or_default();
	provide_context(SubscriptionsCtx(create_rw_signal(subscriptions)));
}

pub fn provide_user_resources() {
	let subs = expect_context::<SubscriptionsCtx>().0;
	let server = expect_context::<ServerCtx>().0.0;

	let subs_ctx = create_resource(
		move || (server.get(), subs.get()),
		|(server, subs)| async move {
			subs.fetch_videos(&server, false).await
		},
	);
	let instances_ctx = create_resource(
		move || (),
		|_| async move {
			fetch_instance_info().await
		},
	);
	let channels_ctx = create_resource(
		move || (server.get(), subs.get()),
		|(server, subs)| async move {
			subs.fetch_channels(&server).await.unwrap()
		},
	);

	create_effect(move |_| {
		subs.track();
		subs_ctx.refetch();
		channels_ctx.refetch();
	});

	provide_context(SubsVideosCtx(subs_ctx));
	provide_context(InstancesCtx(instances_ctx));
	provide_context(ChannelsCtx(channels_ctx));
}
