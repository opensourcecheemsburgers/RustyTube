use gloo::console::debug;
use leptos::{window, Memo, SignalSetter};
use leptos_router::create_query_signal;

pub type VideoQuerySignal = (Memo<Option<String>>, SignalSetter<Option<String>>);
pub fn get_current_video_query_signal() -> VideoQuerySignal {
	create_query_signal("id")
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
