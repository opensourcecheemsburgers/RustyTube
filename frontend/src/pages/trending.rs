use invidious::{Trending, TrendingCategory};
use leptos::*;

use crate::{
	components::{FerrisError, PlaceholderCardArray, VideoPreviewCard},
	contexts::{NetworkConfigCtx, RegionConfigCtx},
	utils::i18n,
};

#[component]
pub fn TrendingSection() -> impl IntoView {
	let region_ctx = expect_context::<RegionConfigCtx>();
	let trending_region = region_ctx.trending_region_slice.0;
	let language = region_ctx.locale_slice.0;
	let category = create_rw_signal(TrendingCategory::Default);
	let server = expect_context::<NetworkConfigCtx>().server_slice.0;
	let trending_resource = create_resource(
		move || {
			(
				server.get(),
				category.get(),
				trending_region.get(),
				language.get().to_invidious_lang(),
			)
		},
		|(server, category, region, lang)| async move {
			Trending::fetch_trending(&server, category, region.alpha2(), &lang).await
		},
	);

	view! {
		<div class="w-full flex justify-center mt-4">
			<div class="w-[90%] flex flex-col gap-y-8">
				<h1 class="font-semibold text-2xl">{i18n("trending.trending")}</h1>
				<TrendingHeader category=category/>
				<Suspense fallback=move || {
					view! { <PlaceholderCardArray/> }
				}>

					{move || {
						trending_resource
							.get()
							.map(|trending_videos_res| {
								match trending_videos_res {
									Ok(trending) => view! { <TrendingVideos trending=trending/> },
									Err(err) => view! { <FerrisError error=err/> },
								}
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
