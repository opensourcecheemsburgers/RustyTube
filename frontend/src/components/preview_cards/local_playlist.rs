use invidious::{CommonPlaylist, LocalPlaylist, LocalPlaylistItem, Video};
use leptos::*;
use leptos_router::NavigateOptions;
use num_format::ToFormattedString;
use phosphor_leptos::{CheckCircle, IconWeight};
use wasm_bindgen::UnwrapThrowExt;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

#[component]
pub fn LocalPlaylistPreviewCard(playlist: LocalPlaylist) -> impl IntoView {
	let server = expect_context::<NetworkConfigCtx>().server_slice.0;

	let playlist = StoredValue::new(playlist.clone());
	let video = Resource::local(
		move || (server.get(), playlist.get_value()),
		|(server, playlist)| async move { playlist.fetch_first_playlist_video(&server).await.unwrap() },
	);

	view! {
		<div class="flex flex-col h-auto overflow-hidden">
			<Suspense fallback=move || {
				().into_view()
			}>

				{move || {
					video
						.get()
						.map(|video| {
							view! {
								<Thumbnail
									id=video.id.clone()
									url=video
										.thumbnails
										.first()
										.cloned()
										.map(|thumb| thumb.url)
										.unwrap_or_default()
									playlist=playlist
								/>
								<Info video=video playlist=playlist/>
							}
						})
				}}

			</Suspense>
		</div>
	}
}

#[component]
pub fn Info(video: Video, playlist: StoredValue<LocalPlaylist>) -> impl IntoView {
	let locale = expect_context::<RegionConfigCtx>().locale_slice.0;

	let name = video.title;
	let author = video.author;
	let author_id = video.author_id;
	let video_count =
		move || playlist.get_value().video_count.to_formatted_string(&locale.get().to_num_fmt());

	let go_to_channel_page = move |_| {
		let navigate = leptos_router::use_navigate();

		request_animation_frame(move || {
			_ = navigate(
				&format!(
					"/playlist?title={}&videos=[{{{}}}]",
					playlist.get_value().title,
					playlist
						.get_value()
						.videos
						.into_iter()
						.map(|video| video.id)
						.collect::<Vec<String>>()
						.join(",")
				),
				Default::default(),
			);
		})
	};

	view! {
		<div class="flex flex-col w-full mt-3 space-y-3 px-2 cursor-text">
			<h1 class="font-sans font-semibold text-base line-clamp-2">
				{playlist.get_value().title}
			</h1>
			<div class="flex flex-row flex-wrap items-center font-normal text-sm gap-1">
				<h2 on:click=go_to_channel_page class="cursor-pointer text-primary">
					{author}
				</h2>
				<p>{"•"}</p>
				<p>
					{move || {
						t!(
							"playlist.videos", video_count = video_count(), locale = & locale.get()
							.id()
						)
					}}

				</p>
			</div>
		</div>
	}
}

#[component]
pub fn Thumbnail(id: String, url: String, playlist: StoredValue<LocalPlaylist>) -> impl IntoView {
	let img_loaded = create_rw_signal(false);
	let image_classes = move || match img_loaded.get() {
		true => "w-full aspect-video object-center object-cover bg-neutral rounded-xl".to_string(),
		false => "animate-pulse w-full aspect-video bg-neutral rounded-xl".to_string(),
	};

	let open_playlist = move |_| {
		let navigate = leptos_router::use_navigate();
		let id = id.clone();
		request_animation_frame(move || {
			_ = navigate(
				&format!(
					"/player?id={}&local_playlist_title={}&local_playlist_videos=[{{{}}}]",
					playlist
						.get_value()
						.videos
						.first()
						.map(|video| video.id.clone())
						.unwrap_or_default(),
					playlist.get_value().title,
					playlist
						.get_value()
						.videos
						.into_iter()
						.map(|video| video.id)
						.collect::<Vec<String>>()
						.join(",")
				),
				Default::default(),
			);
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
