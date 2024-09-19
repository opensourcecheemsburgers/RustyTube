use invidious::VideoShort;
use leptos::*;
use leptos_router::create_query_signal;

use crate::{components::FerrisError, resources::VideoResource, utils::i18n};

#[component]
pub fn RecommendedSection() -> impl IntoView {
	let recommended_view = move || {
		expect_context::<VideoResource>().resource.get().map(|res| match res {
			Ok(video) => video
				.recommended_videos
				.into_iter()
				.map(|video_short| {
					view! { <RecommendedVideo video=video_short/> }
				})
				.collect_view(),
			Err(err) => view! { <FerrisError error=err/> },
		})
	};

	view! {
		<div class="flex flex-col p-4 space-y-4 h-auto rounded-lg bg-base-200">
			<h1 class="text-xl font-semibold">
				{i18n("video.info.recommended")}
			</h1>
			<div class="flex flex-col space-y-4 rounded-lg ltr:pr-4 rtl:pl-4 bg-base-200">
				<Suspense fallback=move || {
					view! { <RecommendedSectionPlaceholder/> }
				}>{recommended_view}</Suspense>
			</div>
		</div>
	}
}

#[component]
pub fn RecommendedSectionCollapsible() -> impl IntoView {
	let recommended_view = move || {
		expect_context::<VideoResource>().resource.get().map(|res| match res {
			Ok(video) => video
				.recommended_videos
				.into_iter()
				.map(|video_short| {
					view! { <RecommendedVideo video=video_short/> }
				})
				.collect_view(),
			Err(err) => view! { <FerrisError error=err/> },
		})
	};

	view! {
		<div>
			<div class="hidden flex-col p-4 space-y-4 h-auto rounded-lg bg-base-200 lg:!flex">
				<h1 class="text-xl font-semibold">
					{i18n("video.info.recommended")}
				</h1>
				<div class="flex flex-col space-y-4 rounded-lg ltr:pr-4 rtl:pl-4 bg-base-200">
					<Suspense fallback=move || {
						view! { <RecommendedSectionPlaceholder/> }
					}>{recommended_view}</Suspense>
				</div>
			</div>

			<div class="rounded-lg lg:hidden collapse collapse-arrow bg-base-200">
				<input type="checkbox"/>
				<div class="text-xl font-medium collapse-title">
					<span>{i18n("video.info.recommended")}</span>

				</div>
				<div class="collapse-content">
					<div class="flex flex-col space-y-4 rounded-lg ltr:pr-4 rtl:pl-4 bg-base-200">
						<Suspense fallback=move || {
							view! { <RecommendedSectionPlaceholder/> }
						}>{recommended_view}</Suspense>
					</div>
				</div>
			</div>
		</div>
	}
}

#[component]
pub fn RecommendedVideo(video: VideoShort) -> impl IntoView {
	let src = video
		.thumbnails
		.get(4)
		.map_or(String::new(), |thumb| thumb.url.clone());

	let video_id = video.id;
	let open_video = move |_| {
		create_query_signal::<String>("id").1.set(Some(video_id.clone()));
	};

	let img_loaded = create_rw_signal(false);
	let image_classes = move || {
		if img_loaded.get() {
			"w-[30%] aspect-video object-center object-cover bg-neutral rounded-lg".to_string()
		} else {
			"animate-pulse w-[30%] aspect-video bg-neutral rounded-lg"
				.to_string()
		}
	};

	view! {
		<div class="flex flex-row gap-x-4">
			<img
				on:click=open_video
				on:load=move |_| img_loaded.set(true)
				src=src
				class=image_classes
			/>
			<div class="flex flex-col w-[70%]">
				<p class="text-sm">{video.title}</p>
				<div class="flex flex-row flex-wrap gap-x-1 mt-2 text-sm">
					<p>{video.author}</p>
					<p>{"•"}</p>
					<p>{video.views_text}</p>
				</div>
			</div>
		</div>
	}
}

#[component]
pub fn RecommendedVideoPlaceholder() -> impl IntoView {
	view! {
		<div class="flex flex-col p-4 rounded-lg bg-base-200">
			<div class="flex flex-col pr-4 space-y-4 rounded-lg">
				<div class="flex flex-row gap-x-6">
					<div class="object-cover object-center rounded-xl animate-pulse bg-neutral aspect-video w-[45%]"></div>
					<div class="flex flex-col space-y-4 w-[55%]">
						<div class="flex flex-col space-y-2">
							<div class="w-full h-3 rounded-xl animate-pulse bg-neutral"></div>
							<div class="h-3 rounded-xl animate-pulse bg-neutral w-[60%]"></div>
						</div>
						<div class="flex flex-row gap-x-2 items-center">
							<div class="h-2 rounded-xl animate-pulse bg-neutral w-[40%]"></div>
							<div class="w-1 h-1 rounded-full animate-pulse bg-neutral"></div>
							<div class="h-2 rounded-xl animate-pulse bg-neutral w-[25%]"></div>
							<div class="w-1 h-1 rounded-full animate-pulse bg-neutral"></div>
							<div class="h-2 rounded-xl animate-pulse bg-neutral w-[20%]"></div>
						</div>
					</div>
				</div>
			</div>
		</div>
	}
}

#[component]
pub fn RecommendedSectionPlaceholder() -> impl IntoView {
	(0..20).map(|_| view! { <RecommendedVideoPlaceholder/> }).collect_view()
}
