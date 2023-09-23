use leptos::{Memo, Scope, SignalSetter};
use leptos_router::create_query_signal;


pub type VideoQuerySignal = (Memo<Option<String>>, SignalSetter<Option<String>>);
pub fn get_current_video_query_signal(cx: Scope) -> VideoQuerySignal {
	create_query_signal(cx, "id")
}