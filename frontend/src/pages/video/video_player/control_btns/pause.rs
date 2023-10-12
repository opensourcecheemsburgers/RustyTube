use leptos::*;
use utils::get_element_by_id;
use web_sys::HtmlAudioElement;

use crate::{
    contexts::{PlaybackState, PlayerState, AUDIO_PLAYER_ID},
    icons::{PauseIcon, PlayIcon},
    utils::is_webkit,
};

#[component]
pub fn PauseBtn() -> impl IntoView {
    let state = expect_context::<PlayerState>();

    let btn_view = move || match state.playback_state.get() {
        PlaybackState::Playing => view! { <PauseIcon/> },
        PlaybackState::Paused => view! { <PlayIcon/> },
        PlaybackState::Loading => view! { <PauseIcon/> },
        PlaybackState::Initial => view! { <PlayIcon/> },
    };

    view! {
        <button
            on:click=move |_| {
                let _ = state.toggle_playback();
            }

            class="btn btn-ghost btn-xs"
        >
            {btn_view}
        </button>
    }
}

