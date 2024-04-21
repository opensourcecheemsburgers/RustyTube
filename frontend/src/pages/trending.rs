use invidious::{Trending, TrendingCategory};
use leptos::*;

use crate::{
	components::{FerrisError, PlaceholderCardArray, VideoPreviewCard},
	contexts::{NetworkConfigCtx, RegionConfigCtx},
	resources::TrendingResource,
	utils::i18n,
};

#[component]
pub fn TrendingSection() -> impl IntoView {
	let category = RwSignal::new(TrendingCategory::Default);
	let trending = TrendingResource::initialise(category);

	view! {
		<div class="w-full flex justify-center mt-4">
			<div class="w-[90%] flex flex-col gap-y-8">
				<h1 class="font-semibold text-2xl">{i18n("trending.trending")}</h1>
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
										<div class="-ml-4 flex flex-row flex-wrap gap-y-12 h-[calc(100vh-15.75rem)] pb-12 overflow-y-hidden hover:overflow-y-auto scroll-smooth">
											<For
												each=move || trending.videos.clone()
												key=|video| video.id.clone()
												let:video
											>
												<VideoPreviewCard video=video/>
											</For>
										</div>
									}
										.into_view()
								}
								Err(err) => view! { <FerrisError error=err/> },
							})
					}}

				</Suspense>
			</div>
		</div>
	}
}

#[component]
pub fn TrendingHeader(category: RwSignal<TrendingCategory>) -> impl IntoView {
	let header_btn_classes = "btn btn-sm btn-outline font-normal normal-case rounded-lg";

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
		<div class="-ml-4 flex flex-row flex-wrap gap-y-12 h-[calc(100vh-15.75rem)] pb-12 overflow-y-hidden hover:overflow-y-auto scroll-smooth">
			<For each=move || trending.videos.clone() key=|video| video.id.clone() let:video>
				<VideoPreviewCard video=video/>
			</For>
		</div>
	}
}
