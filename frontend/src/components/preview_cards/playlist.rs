use invidious::CommonPlaylist;
use leptos::*;
use leptos_router::NavigateOptions;
use num_format::ToFormattedString;
use phosphor_leptos::{CheckCircle, IconWeight};

use crate::{contexts::RegionConfigCtx, utils::go_to};

#[component]
pub fn PlaylistPreviewCard(playlist: CommonPlaylist) -> impl IntoView {
	let playlist_clone = playlist.clone();
	let thumbnail_url = playlist_clone.thumbnail;

	view! {
		<div class="flex flex-col h-auto overflow-hidden">
			<Thumbnail
				playlist_id=playlist.author_id.clone()
				url=thumbnail_url
			/>
			<Info playlist=playlist/>
		</div>
	}
}

#[component]
pub fn Info(playlist: CommonPlaylist) -> impl IntoView {
	let locale = expect_context::<RegionConfigCtx>().locale_slice.0;

	let name = playlist.title;
	let author = playlist.author;
	let author_id = playlist.author_id;
	let video_count = move || {
		playlist.video_count.to_formatted_string(&locale.get().to_num_fmt())
	};
	let verified_check = playlist.author_verified.then_some(
		view! { <CheckCircle weight=IconWeight::Regular class="h-4 w-4 base-content"/> },
	);

	let go_to_channel_page = move |_| go_to(format!("/channel?id={author_id}"));

	view! {
		<div class="flex flex-col w-full mt-3 space-y-3 px-2 cursor-text">
			<h1 class="font-sans font-semibold text-base line-clamp-2">
				{name}
			</h1>
			<div class="flex flex-row flex-wrap items-center font-normal text-sm gap-1">
				<h2
					on:click=go_to_channel_page
					class="cursor-pointer text-primary"
				>
					{author}
				</h2>
				{verified_check}
				<p>{"•"}</p>
				<p>
					{move || {
						t!(
							"playlist.videos", video_count = video_count(), locale = &
							locale.get().id()
						)
					}}

				</p>
			</div>
		</div>
	}
}

#[component]
pub fn Thumbnail(playlist_id: String, url: String) -> impl IntoView {
	let img_loaded = create_rw_signal(false);
	let image_classes = move || {
		if img_loaded.get() {
			"w-full aspect-video object-center object-cover bg-neutral rounded-xl".to_string()
		} else {
			"animate-pulse w-full aspect-video bg-neutral rounded-xl"
				.to_string()
		}
	};

	let open_playlist = move |_| go_to(format!("playlist?id={playlist_id}"));

	view! {
		<div
			on:click=open_playlist
			class="w-full max-w-full overflow-hidden rounded-xl"
		>
			<img
				decoding="async"
				on:load=move |_| img_loaded.set(true)
				src=url
				class=image_classes
			/>
		</div>
	}
}
