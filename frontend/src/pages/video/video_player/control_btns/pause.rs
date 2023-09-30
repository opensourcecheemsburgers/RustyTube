use leptos::*;

use crate::{icons::{PauseIcon, PlayIcon}, contexts::{PlayerState, PlaybackState}};

#[component]
pub fn PauseBtn() -> impl IntoView {
    let state = expect_context::<PlayerState>();

    let toggle_playback = create_action(|state: &PlayerState| {
        let state = state.clone();
        async move {
            state.toggle_playback().await;
        }
    });

    let btn_view = move || match state.playback_state.get() {
        PlaybackState::Playing => view! { <PauseIcon/> },
        PlaybackState::Paused =>  view! { <PlayIcon/> },
        PlaybackState::Loading => view! { <PauseIcon/> },
    };

    view! {
        <button on:click=move |_| toggle_playback.dispatch(state) class="btn btn-ghost btn-xs">
            {btn_view}
        </button>
    }
}

