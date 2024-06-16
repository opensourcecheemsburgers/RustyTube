use leptos::{component, expect_context, view, IntoView, Props};

use crate::contexts::PlayerState;

#[component]
pub fn TimeInfo() -> impl IntoView {
	let state = expect_context::<PlayerState>();

	view! {
		<div class="flex flex-row items-center gap-x-0.5 md:gap-x-1 lg:gap-x-2 ml-2 text-xs md:text-sm lg:text-base font-mono">
			<p>{state.current_time_str}</p>
			<p>/</p>
			<p>{state.duration_str}</p>
		</div>
	}
}
