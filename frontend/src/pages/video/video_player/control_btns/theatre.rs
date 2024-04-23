use leptos::*;
use phosphor_leptos::{ArrowsOut, IconWeight};

use crate::contexts::PlayerStyle;

#[component]
pub fn FullWindowBtn() -> impl IntoView {
	let style = expect_context::<PlayerStyle>();
	let toggle_fullwindow = move |_| style.full_window.set(!style.full_window.get());

	view! {
		<button on:click=toggle_fullwindow class="btn btn-ghost btn-xs">
			<ArrowsOut weight=IconWeight::Regular class="h-4 w-4 base-content"/>
		</button>
	}
}
