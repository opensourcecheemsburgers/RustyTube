use leptos::*;
use phosphor_leptos::{
	IconWeight, SpeakerSimpleHigh, SpeakerSimpleLow, SpeakerSimpleNone, SpeakerSimpleX,
};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

use crate::contexts::PlayerState;

#[component]
pub fn VolumeKnob() -> impl IntoView {
	let state = expect_context::<PlayerState>();
	let volume = move || (state.volume.get() * 100f64).round() as u8;

	let vol_change = move |event: Event| {
		let range = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
		let vol = range.value().parse::<f64>().unwrap_or_default() / 100f64;
		let _ = state.set_volume(vol);
	};

	let knob_visible = RwSignal::new(false);
	let toggle_knob = move |_| knob_visible.set(!knob_visible.get());

	let classes = move || match knob_visible.get() {
		true => "range range-primary range-xs",
		false => "hidden",
	};

	view! {
		<div class="flex flex-row group items-center peer cursor-pointer">
			<button on:click=toggle_knob class="btn btn-ghost btn-xs peer" id="vol_btn">
				{move || match volume() == 0 {
					true => {
						view! {
							<SpeakerSimpleX
								weight=IconWeight::Regular
								class="h-4 w-4 base-content"
							/>
						}
					}
					false => {
						match (1..=20).contains(&volume()) {
							true => {
								view! {
									<SpeakerSimpleNone
										weight=IconWeight::Regular
										class="h-4 w-4 base-content"
									/>
								}
							}
							false => {
								match (21..=50).contains(&volume()) {
									true => {
										view! {
											<SpeakerSimpleLow
												weight=IconWeight::Regular
												class="h-4 w-4 base-content"
											/>
										}
									}
									false => {
										view! {
											<SpeakerSimpleHigh
												weight=IconWeight::Regular
												class="h-4 w-4 base-content"
											/>
										}
									}
								}
							}
						}
					}
				}}

			// <SpeakerNone/>
			// </Show>
			// <Show when=move || state.volume.get() == 0f64>
			// <SpeakerNone/>
			// </Show>
			</button>
			<input
				on:input=vol_change
				id="vol_knob"
				type="range"
				min="0"
				max="100"
				value=volume
				class=classes
			/>
		</div>
	}
}
