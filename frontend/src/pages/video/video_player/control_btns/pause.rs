use leptos::{component, expect_context, view, IntoView, Props, SignalGet};
use phosphor_leptos::{IconWeight, Pause, Play};

use crate::contexts::{PlaybackState, PlayerState};

#[component]
pub fn PauseBtn() -> impl IntoView {
	let state = expect_context::<PlayerState>();

	view! {
		<button
			on:click=move |_| {
				state.toggle_playback();
			}

			class="btn btn-ghost btn-xs lg:btn-sm"
		>
			{move || match state.playback_state.get() {
				PlaybackState::Loading | PlaybackState::Playing => {
					view! {
						<Pause
							weight=IconWeight::Regular
							class="w-4 h-4 lg:w-5 lg:h-5 base-content"
						/>
					}
				}
				PlaybackState::Paused | PlaybackState::Initial => {
					view! {
						<Play
							weight=IconWeight::Regular
							class="w-4 h-4 lg:w-5 lg:h-5 base-content"
						/>
					}
				}
			}}

		</button>
	}
}
