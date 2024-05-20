use invidious::VideoShort;
use leptos::*;
use leptos_router::create_query_signal;
use phosphor_leptos::{Eye, IconWeight};

#[component]
pub fn RecommendedPreviewCard(video: VideoShort) -> impl IntoView {
	let thumbnail_url = video.thumbnails.get(4).map(|thumb| thumb.url.clone());

	let video_id = video.id;
	let open_video = move |_| {
		create_query_signal("id").1.set(Some(video_id.clone()));
	};

	view! {
		<div on:click=open_video class="flex flex-row gap-x-4">
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
			<div class="flex flex-row flex-wrap items-center mt-2 gap-x-1 text-sm">
				<Eye weight=IconWeight::Regular class="h-4 w-4 base-content"/>
				<p>{video.views_text}</p>
			</div>
		</div>
	}
}
