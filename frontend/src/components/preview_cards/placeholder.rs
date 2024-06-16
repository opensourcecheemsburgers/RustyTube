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
		<div class="animate-pulse flex flex-col h-auto overflow-hidden">
			<div class="w-full aspect-video bg-neutral rounded-xl"></div>
			<div class="flex flex-col w-full mt-3 space-y-3 px-2">
				<div class="w-full h-2 rounded-xl bg-neutral"></div>
				<div class="w-full h-2 rounded-xl bg-neutral"></div>
				<div class="w-[35%] h-2 rounded-xl bg-neutral"></div>
			</div>
		</div>
	}
}
