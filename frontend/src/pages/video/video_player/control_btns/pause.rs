use leptos::*;

use crate::{
	contexts::{PlaybackState, PlayerState},
	icons::{PauseIcon, PlayIcon},
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
