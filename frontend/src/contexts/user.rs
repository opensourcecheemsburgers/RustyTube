use invidious::{CommonVideo, Subscriptions};
use leptos::{create_rw_signal, provide_context, RwSignal, Scope};
use wasm_bindgen_futures::spawn_local;

#[derive(Copy, Clone)]
pub struct SubscriptionsCtx(pub RwSignal<Subscriptions>);

#[derive(Copy, Clone)]
pub struct SubsVideosCtx(pub RwSignal<Vec<CommonVideo>>);

pub fn provide_user_contexts(cx: Scope) {
	let subscriptions = Subscriptions::load().unwrap_or_default();
	provide_context(cx, SubscriptionsCtx(create_rw_signal(cx, subscriptions)));
}