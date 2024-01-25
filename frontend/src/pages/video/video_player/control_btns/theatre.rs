use leptos::*;

use crate::{contexts::PlayerStyle, icons::FullWindowIcon};

#[component]
pub fn FullWindowBtn() -> impl IntoView {
	let style = expect_context::<PlayerStyle>();
	let toggle_fullwindow = move |_| style.full_window.set(!style.full_window.get());

	view! {
		<button on:click=toggle_fullwindow class="btn btn-ghost btn-xs">
			<FullWindowIcon/>
		</button>
	}
}
