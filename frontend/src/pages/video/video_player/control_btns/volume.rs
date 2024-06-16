use std::ops::{Mul, RangeBounds};

use leptos::{
	component, expect_context, view, wasm_bindgen, web_sys, IntoView, Props,
	RwSignal, Show, Signal, SignalGet, SignalSet,
};
use phosphor_leptos::{
	IconWeight, SpeakerSimpleHigh, SpeakerSimpleLow, SpeakerSimpleNone,
	SpeakerSimpleX,
};
use rustytube_error::RustyTubeError;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, MouseEvent};

use crate::contexts::PlayerState;

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
#[component]
pub fn VolumeKnob() -> impl IntoView {
	let volume = Signal::derive(|| {
		expect_context::<PlayerState>().volume.get().mul(100f64) as u8
	});

	let knob_visible = RwSignal::new(false);
	let toggle_knob = move |_| knob_visible.set(!knob_visible.get());

	let classes = move || {
		if knob_visible.get() {
			"range range-primary range-xs"
		} else {
			"hidden"
		}
	};

	view! {
		<div class="flex flex-row group items-center peer cursor-pointer">
			<button
				on:click=toggle_knob
				class="btn btn-ghost btn-xs lg:btn-sm peer"
				id="vol_btn"
			>
				<Show when=move || volume.get() == 0>
					<SpeakerSimpleX
						weight=IconWeight::Regular
						class="h-4 w-4 lg:h-5 lg:w-5 base-content"
					/>
				</Show>
				<Show when=move || (1..=20).contains(&volume.get())>
					<SpeakerSimpleNone
						weight=IconWeight::Regular
						class="h-4 w-4 lg:h-5 lg:w-5 base-content"
					/>
				</Show>
				<Show when=move || (21..=50).contains(&volume.get())>
					<SpeakerSimpleLow
						weight=IconWeight::Regular
						class="h-4 w-4 lg:h-5 lg:w-5 base-content"
					/>
				</Show>
				<Show when=move || (51..=100).contains(&volume.get())>
					<SpeakerSimpleHigh
						weight=IconWeight::Regular
						class="h-4 w-4 lg:h-5 lg:w-5 base-content"
					/>
				</Show>
			</button>
			<input
				on:input=change_volume
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

fn change_volume(event: Event) {
	let state = expect_context::<PlayerState>();
	if let Some(range) = event.target() {
		if let Ok(range) = range.dyn_into::<HtmlInputElement>() {
			let vol = range.value().parse::<f64>().unwrap_or_default() / 100f64;
			state.set_volume(vol);
		}
	}
}
