use leptos::{
	component, expect_context, view, IntoView, Props, SignalGet, SignalSet,
};
use phosphor_leptos::{ArrowsOut, IconWeight};

use crate::contexts::PlayerStyle;

#[component]
pub fn FullWindowBtn() -> impl IntoView {
	let style = expect_context::<PlayerStyle>();
	let toggle_fullwindow =
		move |_| style.full_window.set(!style.full_window.get());

	view! {
		<button
			on:click=toggle_fullwindow
			class="btn btn-ghost btn-xs lg:btn-sm"
		>
			<ArrowsOut
				weight=IconWeight::Regular
				class="w-4 h-4 lg:w-5 lg:h-5 base-content"
			/>
		</button>
	}
}
