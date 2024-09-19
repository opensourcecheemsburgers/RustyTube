use invidious::PopularItem;
use leptos::*;
use num_format::ToFormattedString;
use phosphor_leptos::{Eye, IconWeight};

use crate::{contexts::RegionConfigCtx, utils::go_to};

#[component]
pub fn PopularPreviewCard(video: PopularItem) -> impl IntoView {
	view! {
		<div class="flex overflow-hidden flex-col h-auto">
			<Thumbnail
				video_id=video.clone().id
				url=video.thumbnails.get(3).map(|thumb| thumb.url.clone())
			/>
			<Info video=video/>
		</div>
	}
}

#[component]
pub fn Info(video: PopularItem) -> impl IntoView {
	let locale = expect_context::<RegionConfigCtx>().locale_slice.0;

	let title = video.title;
	let author = video.author;
	let published = video.published_text;
	let view_count =
		video.views.to_formatted_string(&locale.get().to_num_fmt());

	let author_id = video.author_id;
	let go_to_channel_page = move |_| {
		let navigate = leptos_router::use_navigate();
		let author_id = author_id.clone();
		go_to(format!("/channel?id={author_id}"));
	};

	view! {
		<div class="flex flex-col px-2 mt-3 space-y-3 w-full cursor-text">
			<h1 class="font-sans text-base font-semibold line-clamp-2">
				{title}
			</h1>
			<div class="flex flex-row flex-wrap gap-1 items-center text-sm font-normal">
				<h2
					on:click=go_to_channel_page
					class="cursor-pointer text-primary"
				>
					{author}
				</h2>
				<p>{"•"}</p>
				<Eye weight=IconWeight::Regular class="w-4 h-4 base-content"/>
				<p>{view_count}</p>
				<p>{"•"}</p>
				<p>{published}</p>
			</div>
		</div>
	}
}

#[component]
pub fn Thumbnail(video_id: String, url: Option<String>) -> impl IntoView {
	let img_loaded = create_rw_signal(false);
	let image_classes = move || {
		if img_loaded.get() {
			"w-full aspect-video object-center object-cover bg-neutral rounded-xl".to_string()
		} else {
			"animate-pulse w-full aspect-video bg-neutral rounded-xl"
				.to_string()
		}
	};

	let open_video = move |_| {
		let navigate = leptos_router::use_navigate();
		let id = video_id.clone();
		go_to(format!("/player?id={id}"));
	};

	view! {
		<div
			on:click=open_video
			class="overflow-hidden w-full max-w-full rounded-xl"
		>
			<img
				decoding="sync"
				on:load=move |_| img_loaded.set(true)
				src=url
				class=image_classes
			/>
		</div>
	}
}
