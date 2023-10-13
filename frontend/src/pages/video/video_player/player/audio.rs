use invidious::{AudioFormat, Container, Format, Formats};
use leptos::{html::audio, *};

use crate::{
    contexts::{PlayerConfigCtx, PlayerState, AUDIO_PLAYER_ID},
    utils::is_webkit,
};

#[component]
pub fn AudioStream() -> impl IntoView {
    let state = expect_context::<PlayerState>();

    let format: RwSignal<Option<Format>> = expect_context::<RwSignal<Option<Format>>>();
    let source = move || format.get().map(|format| format.audio_url()).flatten();

    view! {
        <audio
            on:waiting=move |_| {
                state.set_audio_ready(false);
            }

            on:loadedmetadata=move |_| {
                if is_webkit() {
                    state.set_audio_ready(true);
                }
            }

            on:canplay=move |_| {
                state.set_audio_ready(true);
            }

            id=AUDIO_PLAYER_ID
            preload="auto"
            controls=false
            autoplay=false
            playsinline=true
            src=source
        ></audio>
    }
}

