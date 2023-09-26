use leptos::*;

use crate::{icons::{PauseIcon, PlayIcon}, contexts::{PlayerState, PlaybackState}};

#[component]
pub fn PauseBtn(cx: Scope) -> impl IntoView {
    let state = expect_context::<PlayerState>(cx);

    let toggle_playback = create_action(cx, |state: &PlayerState| {
        let state = state.clone();
        async move {
            state.toggle_playback().await;
        }
    });

    let btn_view = move || match state.playback_state.get() {
        PlaybackState::Playing => view! {cx, <PauseIcon />},
        PlaybackState::Paused =>  view! {cx, <PlayIcon />},
        PlaybackState::Loading => view! {cx, <PauseIcon />},
    };

    view! {cx, 
        <button on:click=move |_| toggle_playback.dispatch(state) class="btn btn-ghost btn-xs">
            {btn_view}
        </button> 
    }
}