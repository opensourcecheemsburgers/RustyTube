use invidious::CommonPlaylist;
use leptos::*;
use leptos_router::NavigateOptions;
use num_format::{Locale, ToFormattedString};

use crate::icons::VerifiedIcon;

#[component]
pub fn PlaylistPreviewCard(playlist: CommonPlaylist) -> impl IntoView {
	let playlist_clone = playlist.clone();
	let thumbnail_url = playlist_clone.thumbnail;

	view! {
		<div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden">
			<Thumbnail playlist_id=playlist.author_id.clone() url=thumbnail_url/>
			<Info playlist=playlist/>
		</div>
	}
}

#[component]
pub fn Info(playlist: CommonPlaylist) -> impl IntoView {
	let name = playlist.title;
	let author = playlist.author;
	let author_id = playlist.author_id;
	let video_count = playlist.video_count.to_formatted_string(&Locale::en);
	let verified_check = playlist.author_verified.then_some(view! { <VerifiedIcon/> });

	let go_to_channel_page = move |_| {
		let navigate = leptos_router::use_navigate();
		let author_id = author_id.clone();
		request_animation_frame(move || {
			_ = navigate(&format!("/channel?id={}", author_id), Default::default());
		})
	};

	view! {
		<div class="flex flex-col w-full mt-3 space-y-3 px-2 cursor-text">
			<h1 class="font-sans font-semibold text-base line-clamp-2">{name}</h1>
			<div class="flex flex-row flex-wrap items-center font-normal text-sm gap-1">
				<h2 on:click=go_to_channel_page class="cursor-pointer text-primary">
					{author}
				</h2>
				{verified_check}
				<p>{"•"}</p>
				<p>{video_count} {r#" videos"#}</p>
			</div>
		</div>
	}
}

#[component]
pub fn Thumbnail(playlist_id: String, url: String) -> impl IntoView {
	let img_loaded = create_rw_signal(false);
	let image_classes = move || match img_loaded.get() {
		true => "w-full aspect-video object-center object-cover bg-neutral rounded-xl".to_string(),
		false => "animate-pulse w-full aspect-video bg-neutral rounded-xl".to_string(),
	};

	let open_playlist = move |_| {
		let navigate = leptos_router::use_navigate();
		let id = playlist_id.clone();
		request_animation_frame(move || {
			_ = navigate(&format!("/playlist?id={}", id), NavigateOptions::default());
		})
	};

	view! {
		<div on:click=open_playlist class="w-full max-w-full overflow-hidden rounded-xl">
			<img
				decoding="async"
				on:load=move |_| img_loaded.set(true)
				src=url
				class=image_classes
			/>
		</div>
	}
}
