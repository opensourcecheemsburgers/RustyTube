use config::Config;
use invidious::{AudioFormat, Container, Format, Formats, Video, VideoFormat};
use leptos::{html::audio, *};
use rustytube_error::RustyTubeError;

use crate::{
    contexts::{PlayerState, PlayerStyle, ServerCtx, AUDIO_PLAYER_ID, VIDEO_PLAYER_ID},
    utils::is_webkit,
};

#[component]
pub fn VideoStream(video: Video) -> impl IntoView {
    let server = expect_context::<ServerCtx>().0 .0;
    let state = expect_context::<PlayerState>();
    let style = expect_context::<PlayerStyle>();

    view! {
        <video
            data-fullwindow=move || style.full_window.get().to_string()
            data-fullscreen=move || style.fullscreen.get().to_string()
            on:waiting=move |_| {
                state.set_video_ready(false);
            }

            on:loadedmetadata=move |_| {
                if is_webkit() {
                    state.set_video_ready(true);
                }
            }

            on:canplay=move |_| {
                state.set_video_ready(true);
            }

            class="w-full rounded max-h-[calc(100vh-12rem)] data-[fullwindow=true]:max-h-screen data-[fullscreen=true]:max-h-screen"
            id=VIDEO_PLAYER_ID
            on:timeupdate=move |_| {
                state.update_time();
            }

            poster=&video.thumbnails.first().unwrap().url
            preload="auto"
            controls=false
            autoplay=false
            playsinline=true
        >
            <VideoSource/>
        </video>
    }
}

#[component]
pub fn VideoSource() -> impl IntoView {
    let format = expect_context::<RwSignal<Option<Format>>>();
    let source = move || format.get().map(|format| format.video_url()).flatten();

    move || view! { <source src=source/> }
}

