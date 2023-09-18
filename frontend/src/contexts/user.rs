use invidious::{Channel, CommonVideo, fetch_instance_info, Instances, Subscriptions, SubscriptionsFetch};
use leptos::{create_effect, create_resource, create_rw_signal, expect_context, provide_context, Resource, RwSignal, Scope, SignalGet, SignalUpdate, SignalWith};
use wasm_bindgen_futures::spawn_local;
use rustytube_error::RustyTubeError;
use crate::contexts::ServerCtx;

#[derive(Copy, Clone)]
pub struct SubscriptionsCtx(pub RwSignal<Subscriptions>);

#[derive(Copy, Clone)]
pub struct ChannelsCtx(pub Resource<(String, Subscriptions), Vec<Result<Channel, RustyTubeError>>>);

#[derive(Copy, Clone)]
pub struct SubsVideosCtx(pub Resource<(String, Subscriptions), SubscriptionsFetch>);

#[derive(Copy, Clone)]
pub struct InstancesCtx(pub Resource<(), Result<Instances, RustyTubeError>>);

pub fn provide_user_contexts(cx: Scope) {
	let subscriptions = Subscriptions::load().unwrap_or_default();
	provide_context(cx, SubscriptionsCtx(create_rw_signal(cx, subscriptions)));
}

pub fn provide_user_resources(cx: Scope) {
	let subs = expect_context::<SubscriptionsCtx>(cx).0;
	let server = expect_context::<ServerCtx>(cx).0.0;

	let subs_ctx = create_resource(
		cx,
		move || (server.get(), subs.get()),
		|(server, subs)| async move {
			subs.fetch_videos(&server, false).await
		},
	);
	let instances_ctx = create_resource(
		cx,
		move || (),
		|_| async move {
			fetch_instance_info().await
		},
	);
	let channels_ctx = create_resource(
		cx,
		move || (server.get(), subs.get()),
		|(server, subs)| async move {
			subs.fetch_channels(&server).await.unwrap()
		},
	);

	create_effect(cx, move |_| {
		subs.track();
		subs_ctx.refetch();
		channels_ctx.refetch();
	});

	provide_context(cx, SubsVideosCtx(subs_ctx));
	provide_context(cx, InstancesCtx(instances_ctx));
	provide_context(cx, ChannelsCtx(channels_ctx));
}