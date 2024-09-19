use invidious::{Trending, TrendingCategory};
use leptos::{
	component, view, For, IntoView, Props, RwSignal, SignalGet, SignalSet,
	Suspense,
};

use crate::{
	components::{
		CardGrid, FerrisError, GridContainer, PlaceholderCardArray,
		VideoPreviewCard,
	},
	resources::TrendingResource,
	utils::i18n,
};

#[component]
pub fn TrendingSection() -> impl IntoView {
	let category = RwSignal::new(TrendingCategory::Default);
	let trending = TrendingResource::initialise(category);

	view! {
		<GridContainer>
			<h1 class="text-2xl font-semibold">{i18n("trending.trending")}</h1>
			<TrendingHeader category=category/>
			<Suspense fallback=move || {
				view! { <PlaceholderCardArray/> }
			}>
				{move || {
					trending
						.resource
						.get()
						.map(|trending| match trending {
							Ok(trending) => {
								view! {
									<CardGrid>
										<For
											each=move || trending.videos.clone()
											key=|video| video.id.clone()
											let:video
										>
											<VideoPreviewCard video=video/>
										</For>
									</CardGrid>
								}
									.into_view()
							}
							Err(err) => view! { <FerrisError error=err/> },
						})
				}}

			</Suspense>
		</GridContainer>
	}
}

#[component]
pub fn TrendingHeader(category: RwSignal<TrendingCategory>) -> impl IntoView {
	let header_btn_classes =
		"btn btn-sm btn-outline font-normal normal-case rounded-lg";

	view! {
		<div class="flex flex-row gap-x-3">
			<button
				on:click=move |_| category.set(TrendingCategory::Default)
				class=header_btn_classes
			>
				{i18n("trending.all")}
			</button>
			<button
				on:click=move |_| category.set(TrendingCategory::Music)
				class=header_btn_classes
			>
				{i18n("trending.music")}
			</button>
			<button
				on:click=move |_| category.set(TrendingCategory::Gaming)
				class=header_btn_classes
			>
				{i18n("trending.gaming")}
			</button>
			<button
				on:click=move |_| category.set(TrendingCategory::Movies)
				class=header_btn_classes
			>
				{i18n("trending.movies")}
			</button>
		</div>
	}
}

#[component]
pub fn TrendingVideos(trending: Trending) -> impl IntoView {
	view! {
		<div class="overflow-y-hidden hover:overflow-y-auto h-[calc(100vh-15.75rem)] scroll-smooth">
			<CardGrid>
				<For
					each=move || trending.videos.clone()
					key=|video| video.id.clone()
					let:video
				>
					<VideoPreviewCard video=video/>
				</For>
			</CardGrid>
		</div>
	}
}
