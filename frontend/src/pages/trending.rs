use invidious::{CountryCode, Trending, TrendingCategory, TrendingCategory::*};
use leptos::*;

use crate::{
	components::{FerrisError, PlaceholderCardArray, VideoPreviewCard},
	contexts::ServerCtx,
};

#[component]
pub fn TrendingSection() -> impl IntoView {
	let category = create_rw_signal(Default);
	let server = expect_context::<ServerCtx>().0 .0;
	let trending_resource = create_resource(
		move || (server.get(), category.get()),
		|(server, category)| async move {
			Trending::fetch_trending(&server, category, CountryCode::IE).await
		},
	);

	// let trending_content_view = move || trending_resource.read().map(|trending_videos_res| {
	//     match trending_videos_res {
	//         Ok(trending) => view! {<TrendingVideos trending=trending />},
	//         Err(err) => view! {<FerrisError error=err/>}
	//     }
	// }) ;

	view! {
		<div class="w-full flex justify-center mt-4">
			<div class="w-[90%] flex flex-col gap-y-8">
				<h1 class="font-semibold text-2xl">{"Trending"}</h1>
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
			<button on:click=move |_| category.set(Default) class=header_btn_classes>
				All
			</button>
			<button on:click=move |_| category.set(Music) class=header_btn_classes>
				Music
			</button>
			<button on:click=move |_| category.set(Gaming) class=header_btn_classes>
				Gaming
			</button>
			<button on:click=move |_| category.set(Movies) class=header_btn_classes>
				Movies
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
