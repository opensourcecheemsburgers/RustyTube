use leptos::*;
use phosphor_leptos::{FrameCorners, IconWeight};
use utils::get_element_by_id;
use web_sys::HtmlDivElement;

use crate::contexts::{PlayerStyle, VIDEO_CONTAINER_ID};

#[component]
pub fn FullScreenBtn() -> impl IntoView {
	let style = expect_context::<PlayerStyle>();

	let fullscreen = move |_| match document().fullscreen() {
		true => document().exit_fullscreen(),
		false => {
			let fullscreen = get_element_by_id::<HtmlDivElement>(VIDEO_CONTAINER_ID)
				.unwrap()
				.request_fullscreen();
			if fullscreen.is_ok() {
				style.fullscreen.set(true);
			}
		}
	};

	view! {
		<button on:click=fullscreen class="btn btn-ghost btn-xs">
			<FrameCorners weight=IconWeight::Regular class="h-4 w-4 base-content"/>
		</button>
	}
}
