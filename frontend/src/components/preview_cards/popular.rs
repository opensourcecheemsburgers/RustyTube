use invidious::PopularItem;
use leptos::*;
use num_format::{Locale, ToFormattedString};

#[component]
pub fn PopularPreviewCard(video: PopularItem) -> impl IntoView {
	view! {
		<div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden">
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
	let title = video.title;
	let view_count = video.views.to_formatted_string(&Locale::en);
	let author = video.author;
	let published = video.published_text;

	let author_id = video.author_id;
	let go_to_channel_page = move |_| {
		let navigate = leptos_router::use_navigate();
		let author_id = author_id.clone();
		request_animation_frame(move || {
			_ = navigate(&format!("/channel?id={}", author_id), Default::default());
		})
	};

	view! {
		<div class="flex flex-col w-full mt-3 space-y-3 px-2 cursor-text">
			<h1 class="font-sans font-semibold text-base line-clamp-2">{title}</h1>
			<div class="flex flex-row flex-wrap items-center font-normal text-sm gap-1">
				<h2 on:click=go_to_channel_page class="cursor-pointer text-primary">
					{author}
				</h2>
				<p>{"•"}</p>
				<p>{view_count} {r#" views"#}</p>
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
