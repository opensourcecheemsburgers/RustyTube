use leptos::*;

use crate::contexts::PlayerState;

#[component]
pub fn TimeInfo() -> impl IntoView {
	let state = expect_context::<PlayerState>();

	view! {
		<div class="flex flex-row items-center space-x-0.5 md:space-x-1 lg:space-x-2 ml-2 text-xs md:text-sm lg:text-base font-mono">
			<p>{state.current_time_str}</p>
			<p>/</p>
			<p>{state.duration_str}</p>
		</div>
	}
}
