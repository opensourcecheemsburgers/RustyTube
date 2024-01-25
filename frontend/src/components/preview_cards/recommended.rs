use invidious::VideoShort;
use leptos::*;

use crate::utils::get_current_video_query_signal;

#[component]
pub fn RecommendedPreviewCard(video: VideoShort) -> impl IntoView {
	let thumbnail_url = video.thumbnails.get(4).map(|thumb| thumb.url.clone());

	let video_id = video.id;
	let video_id_query_signal_setter = get_current_video_query_signal().1;
	let open_video = move |_| {
		video_id_query_signal_setter.set(Some(video_id.clone()));
	};

	view! {
		<div on:click=open_video class="flex flex-row space-x-4">
			<Thumbnail url=thumbnail_url/>
		</div>
	}
}

#[component]
pub fn Thumbnail(url: Option<String>) -> impl IntoView {
	let img_loaded = create_rw_signal(false);
	let image_classes = move || match img_loaded.get() {
		true => "w-[30%] aspect-video object-center object-cover bg-neutral rounded-lg".to_string(),
		false => "animate-pulse w-[30%] aspect-video bg-neutral rounded-lg".to_string(),
	};

	view! { <img decoding="async" on:load=move |_| img_loaded.set(true) src=url class=image_classes/> }
}

#[component]
pub fn Info(video: VideoShort) -> impl IntoView {
	view! {
		<div class="flex flex-col w-[70%] overflow-hidden">
			<p class="text-sm">{video.title}</p>
			<div class="flex flex-row flex-wrap items-center mt-2 space-x-1 text-sm">
				<p>{video.author}</p>
				<p>{"•"}</p>
				<p>{video.views_text} views</p>
			</div>
		</div>
	}
}
