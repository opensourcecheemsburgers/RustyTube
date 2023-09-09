use leptos::{create_rw_signal, provide_context, RwSignal, Scope};
use invidious::VideoFormat;

#[derive(Copy, Clone)]
pub struct VideoIdCtx(pub RwSignal<String>);

#[derive(Copy, Clone)]
pub struct VideoFormatCtx(pub RwSignal<VideoFormat>);

pub fn provide_player_contexts(cx: Scope) {
	provide_context(cx, VideoIdCtx(create_rw_signal(cx, "".to_string())));
}