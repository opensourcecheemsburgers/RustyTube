use leptos::*;

use crate::contexts::PlayerState;

#[component]
pub fn TimeInfo() -> impl IntoView {
	let state = expect_context::<PlayerState>();

	view! {
		<div class="flex flex-row space-x-2 ml-2 font-mono">
			<p>{state.current_time_str}</p>
			<p>/</p>
			<p>{state.duration_str}</p>
		</div>
	}
}
