use invidious::VideoShort;
use leptos::*;

use crate::{
	components::FerrisError, pages::video::page::VideoResource,
	utils::get_current_video_query_signal,
};

#[component]
pub fn RecommendedSection(video_resource: VideoResource) -> impl IntoView {
	let recommended_view = move || {
		video_resource.get().map(|res| match res {
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
		<div class="flex flex-col rounded-lg bg-base-200 p-4 space-y-4">
			<h1 class="font-semibold text-xl">Recommended</h1>
			<div class="flex flex-col space-y-4 pr-4 rounded-lg bg-base-200">
				<Suspense fallback=move || {
					view! { <RecommendedSectionPlaceholder/> }
				}>{recommended_view}</Suspense>
			</div>
		</div>
	}
}
#[component]
pub fn RecommendedVideo(video: VideoShort) -> impl IntoView {
	let src = video.thumbnails.get(4).cloned().unwrap().url;

	let video_id = video.id;
	let video_id_setter = get_current_video_query_signal().1;
	let open_video = move |_| {
		video_id_setter.set(Some(video_id.clone()));
	};

	let img_loaded = create_rw_signal(false);
	let image_classes = move || match img_loaded.get() {
		true => "w-[30%] aspect-video object-center object-cover bg-neutral rounded-lg".to_string(),
		false => "animate-pulse w-[30%] aspect-video bg-neutral rounded-lg".to_string(),
	};

	view! {
		<div class="flex flex-row space-x-4">
			<img
				on:click=open_video
				on:load=move |_| img_loaded.set(true)
				src=src
				class=image_classes
			/>
			<div class="flex flex-col w-[70%]">
				<p class="text-sm">{video.title}</p>
				<div class="flex flex-row flex-wrap mt-2 space-x-1 text-sm">
					<p>{video.author}</p>
					<p>{"•"}</p>
					<p>{video.views_text} views</p>
				</div>
			</div>
		</div>
	}
}

#[component]
pub fn RecommendedVideoPlaceholder() -> impl IntoView {
	view! {
		<div class="bg-base-200 flex flex-col rounded-lg p-4">
			<div class="flex flex-col space-y-4 rounded-lg pr-4">
				<div class="flex flex-row space-x-6">
					<div class="bg-neutral aspect-video w-[45%] animate-pulse rounded-xl object-cover object-center"></div>
					<div class="flex w-[55%] flex-col space-y-4">
						<div class="flex flex-col space-y-2">
							<div class="bg-neutral h-3 w-full animate-pulse rounded-xl"></div>
							<div class="bg-neutral h-3 w-[60%] animate-pulse rounded-xl"></div>
						</div>
						<div class="flex flex-row items-center space-x-2">
							<div class="bg-neutral h-2 w-[40%] animate-pulse rounded-xl"></div>
							<div class="bg-neutral h-1 w-1 animate-pulse rounded-full"></div>
							<div class="bg-neutral h-2 w-[25%] animate-pulse rounded-xl"></div>
							<div class="bg-neutral h-1 w-1 animate-pulse rounded-full"></div>
							<div class="bg-neutral h-2 w-[20%] animate-pulse rounded-xl"></div>
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
