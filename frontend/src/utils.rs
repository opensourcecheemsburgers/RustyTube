use gloo::console::debug;
use leptos::{expect_context, request_animation_frame, window, SignalGet};
use leptos_router::{create_query_signal, NavigateOptions};

use crate::contexts::RegionConfigCtx;

pub fn i18n(key: &'static str) -> impl Fn() -> String {
	move || {
		t!(
			key,
			locale =
				&expect_context::<RegionConfigCtx>().locale_slice.0.get().id()
		)
		.to_string()
	}
}

pub fn is_webkit() -> bool {
	window()
		.navigator()
		.user_agent()
		.map_or(false, |user_agent_string| user_agent_string.contains("WebKit"))
}

pub fn go_to(page: impl AsRef<str>) {
	let navigate = leptos_router::use_navigate();
	let page = page.as_ref().to_string();
	request_animation_frame(move || {
		navigate(&page, NavigateOptions::default());
	});
}
