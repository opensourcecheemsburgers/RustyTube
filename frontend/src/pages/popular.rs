use invidious::Popular;
use leptos::*;

use crate::{
	components::{FerrisError, PlaceholderCardArray, PopularPreviewCard},
	resources::PopularResource,
	utils::i18n,
};

#[component]
pub fn PopularSection() -> impl IntoView {
	let popular = PopularResource::initialise();

	view! {
		<div class="w-full flex justify-center mt-4">
			<div class="w-[90%] flex flex-col gap-y-8">
				<h1 class="font-semibold text-2xl">{i18n("sidebar.popular")}</h1>
				<Suspense fallback=move || {
					view! { <PlaceholderCardArray/> }
				}>
					{move || {
						popular
							.resource
							.get()
							.map(|popular_videos_res| {
								match popular_videos_res {
									Ok(popular) => {
										view! { <PopularVideos popular=popular/> }.into_view()
									}
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
pub fn PopularVideos(popular: Popular) -> impl IntoView {
	let popular_videos_view = popular
		.items
		.into_iter()
		.map(|video| view! { <PopularPreviewCard video=video/> })
		.collect_view();

	view! {
		<div class="-ml-4 flex flex-row flex-wrap gap-y-12 h-[calc(100vh-11.75rem)] pb-12 overflow-y-hidden hover:overflow-y-auto scroll-smooth">
			{popular_videos_view}
		</div>
	}
}
