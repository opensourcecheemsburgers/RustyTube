use leptos::*;
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
				PlaybackState::Playing => {
					view! {
						<Pause
							weight=IconWeight::Regular
							class="h-4 w-4 lg:h-5 lg:w-5 base-content"
						/>
					}
				}
				PlaybackState::Loading => {
					view! {
						<Pause
							weight=IconWeight::Regular
							class="h-4 w-4 lg:h-5 lg:w-5 base-content"
						/>
					}
				}
				PlaybackState::Paused => {
					view! {
						<Play
							weight=IconWeight::Regular
							class="h-4 w-4 lg:h-5 lg:w-5 base-content"
						/>
					}
				}
				PlaybackState::Initial => {
					view! {
						<Play
							weight=IconWeight::Regular
							class="h-4 w-4 lg:h-5 lg:w-5 base-content"
						/>
					}
				}
			}}

		</button>
	}
}
