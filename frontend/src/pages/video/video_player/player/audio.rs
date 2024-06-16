use invidious::Format;
use leptos::*;

use crate::{
	components::FerrisError,
	contexts::{PlayerState, AUDIO_PLAYER_ID},
	resources::CaptionsResource,
	utils::is_webkit,
};

#[component]
pub fn AudioStream() -> impl IntoView {
	let state = expect_context::<PlayerState>();

	let format: RwSignal<Option<Format>> =
		expect_context::<RwSignal<Option<Format>>>();
	let source = move || format.get().and_then(|format| format.audio_url());

	let captions = CaptionsResource::initialise();
	provide_context(captions);

	view! {
		<audio
			on:waiting=move |_| {
				let _ = state.set_audio_ready(false);
			}

			on:loadedmetadata=move |_| {
				if is_webkit() {
					let _ = state.set_audio_ready(true);
				}
			}

			on:canplay=move |_| {
				let _ = state.set_audio_ready(true);
			}

			id=AUDIO_PLAYER_ID
			preload="auto"
			controls=false
			autoplay=false
			playsinline=true
			src=source
		>
			{move || {
				captions
					.resource
					.get()
					.map(|captions| match captions {
						Ok(captions) => {
							view! {
								<For
									each=move || captions.captions.clone()
									key=|caption| caption.url.clone()
									let:caption
								>
									<track
										kind="captions"
										srclang=caption.language
										src=caption.url
									/>
								</For>
							}
						}
						Err(err) => view! { <FerrisError error=err/> },
					})
			}}

		</audio>
	}
}
