use invidious::CommonVideo;
use leptos::*;
use num_format::ToFormattedString;
use phosphor_leptos::{CheckCircle, Eye, IconWeight};

use crate::contexts::RegionConfigCtx;

#[component]
pub fn VideoPreviewCard(video: CommonVideo) -> impl IntoView {
	view! {
		<div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden">
			<Thumbnail
				video_id=video.id.clone()
				url=video.thumbnails.get(3).map(|thumb| thumb.url.clone())
			/>
			<Info video=video/>
		</div>
	}
}

#[component]
pub fn Info(video: CommonVideo) -> impl IntoView {
	let locale = expect_context::<RegionConfigCtx>().locale_slice.0;

	let title = video.title;
	let author = video.author;
	let author_id = video.author_id;

	let views = move || video.views.to_formatted_string(&locale.get().to_num_fmt());

	let published = match video.upcoming {
		true => utils::get_time_until(video.premiere_timestamp),
		false => video.published_text,
	};

	let verified_check = video.author_verified.then_some(
		view! { <CheckCircle weight=IconWeight::Regular class="h-4 w-4 base-content"/> },
	);

	let go_to_channel_page = move |_| {
		let navigate = leptos_router::use_navigate();
		let author_id = author_id.clone();
		request_animation_frame(move || {
			_ = navigate(&format!("/channel?id={}", author_id), Default::default());
		})
	};

	view! {
		<div class="flex flex-col w-full mt-3 space-y-3 px-2 cursor-text">
			<h1 class=" font-sans font-semibold text-base line-clamp-2">{title}</h1>
			<div class="flex flex-row flex-wrap items-center font-normal text-sm gap-1">
				<h2 on:click=go_to_channel_page class="cursor-pointer text-primary">
					{author}
				</h2>
				{verified_check}
				<p>{"•"}</p>
				<Eye weight=IconWeight::Regular class="h-4 w-4 base-content"/>
				<p>{views}</p>
				<p>{"•"}</p>
				<p>{published}</p>
			</div>
		</div>
	}
}

#[component]
pub fn Thumbnail(video_id: String, url: Option<String>) -> impl IntoView {
	let img_loaded = create_rw_signal(false);
	let image_classes = move || match img_loaded.get() {
		true => "w-full aspect-video object-center object-cover bg-neutral rounded-xl".to_string(),
		false => "animate-pulse w-full aspect-video bg-neutral rounded-xl".to_string(),
	};

	let open_video = move |_| {
		let navigate = leptos_router::use_navigate();
		let id = video_id.clone();
		request_animation_frame(move || {
			_ = navigate(&format!("/player?id={}", id), Default::default());
		})
	};

	view! {
		<div on:click=open_video class="w-full max-w-full overflow-hidden rounded-xl">
			<img decoding="sync" on:load=move |_| img_loaded.set(true) src=url class=image_classes/>
		</div>
	}
}
