use invidious::{CountryCode, Trending, TrendingCategory, TrendingCategory::*};
use leptos::*;

use crate::{
	components::{FerrisError, PlaceholderCardArray, VideoPreviewCard},
	contexts::{LocaleCtx, ServerCtx, TrendingRegionCtx},
};

#[component]
pub fn TrendingSection() -> impl IntoView {
	let locale = expect_context::<LocaleCtx>().0 .0;
	let region = expect_context::<TrendingRegionCtx>().0 .0;

	let category = create_rw_signal(Default);
	let server = expect_context::<ServerCtx>().0 .0;
	let language = expect_context::<LocaleCtx>().0 .0;
	let trending_resource = create_resource(
		move || (server.get(), category.get(), region.get(), language.get().to_invidious_lang()),
		|(server, category, region, lang)| async move {
			Trending::fetch_trending(&server, category, region.alpha2(), &lang).await
		},
	);

	view! {
		<div class="w-full flex justify-center mt-4">
			<div class="w-[90%] flex flex-col gap-y-8">
				<h1 class="font-semibold text-2xl">
					{move || t!("trending.trending", locale = & locale.get().id())}
				</h1>
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
	let locale = expect_context::<LocaleCtx>().0 .0;

	let header_btn_classes = "btn btn-sm btn-outline font-normal normal-case rounded-lg";

	view! {
		<div class="flex flex-row gap-x-3">
			<button on:click=move |_| category.set(Default) class=header_btn_classes>
				{move || t!("trending.all", locale = & locale.get().id())}
			</button>
			<button on:click=move |_| category.set(Music) class=header_btn_classes>
				{move || t!("trending.music", locale = & locale.get().id())}
			</button>
			<button on:click=move |_| category.set(Gaming) class=header_btn_classes>
				{move || t!("trending.gaming", locale = & locale.get().id())}
			</button>
			<button on:click=move |_| category.set(Movies) class=header_btn_classes>
				{move || t!("trending.movies", locale = & locale.get().id())}
			</button>
		</div>
	}
}

#[component]
pub fn TrendingVideos(trending: Trending) -> impl IntoView {
	let trending_videos_view = trending
		.videos
		.into_iter()
		.map(|video| view! { <VideoPreviewCard video=video/> })
		.collect_view();

	view! {
		<div class="-ml-4 flex flex-row flex-wrap gap-y-12 h-[calc(100vh-15.75rem)] pb-12 overflow-y-hidden hover:overflow-y-auto scroll-smooth">
			{trending_videos_view}
		</div>
	}
}
