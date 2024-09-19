use leptos::{component, view, CollectView, IntoView, Props};

use crate::components::CardGrid;

#[component]
pub fn PlaceholderCardArray() -> impl IntoView {
	view! {
		<CardGrid>
			{(0..50).map(|_| view! { <PlaceholderCard/> }).collect_view()}
		</CardGrid>
	}
}

#[component]
pub fn PlaceholderCard() -> impl IntoView {
	view! {
		<div class="flex overflow-hidden flex-col h-auto animate-pulse">
			<div class="w-full rounded-xl aspect-video bg-neutral"></div>
			<div class="flex flex-col px-2 mt-3 space-y-3 w-full">
				<div class="w-full h-2 rounded-xl bg-neutral"></div>
				<div class="w-full h-2 rounded-xl bg-neutral"></div>
				<div class="h-2 rounded-xl w-[35%] bg-neutral"></div>
			</div>
		</div>
	}
}
