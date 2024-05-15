use gloo::console::debug;
use leptos::*;
use leptos_router::create_query_signal;

use crate::contexts::RegionConfigCtx;

pub fn i18n(key: &'static str) -> impl Fn() -> String {
	move || {
		t!(key, locale = &expect_context::<RegionConfigCtx>().locale_slice.0.get().id()).to_string()
	}
}

pub fn is_webkit() -> bool {
	match window().navigator().user_agent() {
		Ok(user_agent_string) => match user_agent_string.contains("WebKit") {
			true => {
				debug!("Webkit");
				true
			}
			false => false,
		},
		Err(_) => false,
	}
}

pub fn go_to(page: String) {
	let navigate = leptos_router::use_navigate();
	let page = page.clone();
	request_animation_frame(move || {
		_ = navigate(&page, Default::default());
	})
}
