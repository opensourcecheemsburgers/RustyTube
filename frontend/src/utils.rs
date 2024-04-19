use gloo::console::debug;
use leptos::{expect_context, window, Memo, SignalGet, SignalSetter};
use leptos_router::create_query_signal;

use crate::contexts::RegionConfigCtx;

pub type VideoQuerySignal = (Memo<Option<String>>, SignalSetter<Option<String>>);
pub fn get_current_video_query_signal() -> VideoQuerySignal {
	create_query_signal("id")
}

pub fn i18n(key: &'static str) -> impl Fn() -> String {
	move || t!(key, locale = &expect_context::<RegionConfigCtx>().locale_slice.0.get().id()).to_string()
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
