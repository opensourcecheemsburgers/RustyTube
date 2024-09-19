use leptos::{component, expect_context, view, IntoView, Props};

use crate::contexts::PlayerState;

#[component]
pub fn TimeInfo() -> impl IntoView {
	let state = expect_context::<PlayerState>();

	view! {
		<div class="flex flex-row gap-x-0.5 items-center ml-2 font-mono text-xs md:gap-x-1 md:text-sm lg:gap-x-2 lg:text-base">
			<p>{state.current_time_str}</p>
			<p>/</p>
			<p>{state.duration_str}</p>
		</div>
	}
}
