use std::{
	ops::{Mul, RangeBounds},
	u8,
};

use leptos::{
	component, expect_context, html::Input, view, wasm_bindgen, web_sys,
	IntoView, NodeRef, Props, RwSignal, Show, Signal, SignalGet, SignalSet,
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
	let state = expect_context::<PlayerState>();
	let volume_readable =
		Signal::derive(move || state.volume.get().mul(100f64) as u8);

	let knob_visible = RwSignal::new(false);
	let toggle_knob = move |_| knob_visible.set(!knob_visible.get());

	let classes = move || {
		if knob_visible.get() {
			"range range-primary range-xs"
		} else {
			"hidden"
		}
	};

	let vol_ref = NodeRef::<Input>::new();
	let change_volume = move |_| {
		if let Some(vol_node) = vol_ref.get() {
			let vol =
				vol_node.value().parse::<f64>().unwrap_or_default() / 100f64;
			state.set_volume(vol);
		}
	};

	view! {
		<div class="flex flex-row items-center cursor-pointer group peer">
			<button
				on:click=toggle_knob
				class="btn btn-ghost btn-xs peer lg:btn-sm"
				id="vol_btn"
			>
				<Show when=move || volume_readable.get() == u8::MIN>
					<SpeakerSimpleX
						weight=IconWeight::Regular
						class="w-4 h-4 lg:w-5 lg:h-5 base-content"
					/>
				</Show>
				<Show when=move || (1..=20).contains(&volume_readable.get())>
					<SpeakerSimpleNone
						weight=IconWeight::Regular
						class="w-4 h-4 lg:w-5 lg:h-5 base-content"
					/>
				</Show>
				<Show when=move || (21..=50).contains(&volume_readable.get())>
					<SpeakerSimpleLow
						weight=IconWeight::Regular
						class="w-4 h-4 lg:w-5 lg:h-5 base-content"
					/>
				</Show>
				<Show when=move || (51..=100).contains(&volume_readable.get())>
					<SpeakerSimpleHigh
						weight=IconWeight::Regular
						class="w-4 h-4 lg:w-5 lg:h-5 base-content"
					/>
				</Show>
			</button>
			<input
				_ref=vol_ref
				on:input=change_volume
				id="vol_knob"
				type="range"
				min="0"
				max="100"
				value=volume_readable
				class=classes
			/>
		</div>
	}
}
