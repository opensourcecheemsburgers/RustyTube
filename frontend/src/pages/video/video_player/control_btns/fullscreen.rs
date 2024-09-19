use leptos::{
	component, document, expect_context, view, web_sys, window, IntoView,
	Props, SignalSet,
};
use phosphor_leptos::{FrameCorners, IconWeight};
use utils::get_element_by_id;
use web_sys::{HtmlDivElement, OrientationLockType};

use crate::contexts::{PlayerStyle, VIDEO_CONTAINER_ID};

#[component]
pub fn FullScreenBtn() -> impl IntoView {
	let style = expect_context::<PlayerStyle>();

	let fullscreen = move |_| {
		if document().fullscreen() {
			document().exit_fullscreen();
		} else if let Ok(element) =
			get_element_by_id::<HtmlDivElement>(VIDEO_CONTAINER_ID)
		{
			element.request_fullscreen();
		}
	};

	view! {
		<button on:click=fullscreen class="btn btn-ghost btn-xs lg:btn-sm">
			<FrameCorners
				weight=IconWeight::Regular
				class="w-4 h-4 lg:w-5 lg:h-5 base-content"
			/>
		</button>
	}
}
