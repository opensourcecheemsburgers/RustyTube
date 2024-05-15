use invidious::{Caption, Captions};
use leptos::*;
use phosphor_leptos::{IconWeight, Subtitles};
use wasm_bindgen::JsCast;
use web_sys::{HtmlVideoElement, MouseEvent, TextTrackMode};

use crate::{components::FerrisError, contexts::VIDEO_PLAYER_ID, resources::CaptionsResource};

#[component]
pub fn CaptionsDropdown() -> impl IntoView {
	let captions = expect_context::<CaptionsResource>();

	view! {
		<Suspense fallback=move || {
			().into_view()
		}>

			{move || {
				captions
					.resource
					.get()
					.map(|captions| match captions {
						Ok(captions) => {
							view! {
								<div class="dropdown dropdown-top dropdown-end z-20">
									<CaptionsDropdownBtn/>
									<CaptionsDropdownContent captions=captions/>
								</div>
							}
								.into_view()
						}
						Err(err) => view! { <FerrisError error=err/> },
					})
			}}

		</Suspense>
	}
}

#[component]
pub fn CaptionsDropdownBtn() -> impl IntoView {
	view! {
		<label tabindex="0" class="btn btn-ghost btn-xs">
			<Subtitles weight=IconWeight::Regular class="h-4 w-4 base-content"/>
		</label>
	}
}

#[component]
pub fn CaptionsDropdownContent(captions: Captions) -> impl IntoView {
	view! {
		<ul
			tabindex="0"
			class="menu dropdown-content py-3 mb-4 mr-2 shadow bg-base-300 rounded-xl w-max h-auto max-h-48"
		>
			<div class="flex flex-col h-full overflow-y-scroll space-y-2 px-3">
				{captions
					.captions
					.into_iter()
					.map(|caption| {
						view! { <CaptionsDropdownItem caption=caption/> }
					})
					.collect_view()}

			</div>
		</ul>
	}
}

#[component]
pub fn CaptionsDropdownItem(caption: Caption) -> impl IntoView {
	let set_captions = move |_: MouseEvent| {
		let video: HtmlVideoElement =
			document().get_element_by_id(VIDEO_PLAYER_ID).unwrap().dyn_into().unwrap();
		video
			.text_tracks()
			.unwrap()
			.get_track_by_id(&caption.language)
			.unwrap()
			.set_mode(TextTrackMode::Showing);
	};

	view! {
		<div class="p-3 rounded-lg bg-base-100 cursor-pointer">
			<a on:click=set_captions class="text-base-content font-sans">
				{caption.label}
			</a>
		</div>
	}
}
