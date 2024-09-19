use invidious::{LocalPlaylist, Video};
use leptos::*;
use num_format::ToFormattedString;

use crate::{
	contexts::{NetworkConfigCtx, RegionConfigCtx},
	utils::go_to,
};

#[component]
pub fn LocalPlaylistPreviewCard(playlist: LocalPlaylist) -> impl IntoView {
	let server = expect_context::<NetworkConfigCtx>().server_slice.0;

	let playlist = StoredValue::new(playlist);
	let video = Resource::local(
		move || (server.get(), playlist.get_value()),
		|(server, playlist)| async move {
			playlist
				.fetch_first_playlist_video(&server)
				.await
				.expect("First playlist video should load.")
		},
	);

	view! {
		<div class="flex overflow-hidden flex-col h-auto">
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
pub fn Info(
	video: Video,
	playlist: StoredValue<LocalPlaylist>,
) -> impl IntoView {
	let locale = expect_context::<RegionConfigCtx>().locale_slice.0;

	let name = video.title;
	let author = video.author;
	let author_id = video.author_id;
	let video_count = move || {
		playlist
			.get_value()
			.video_count
			.to_formatted_string(&locale.get().to_num_fmt())
	};

	let go_to_local_playlist_page = move |_| {
		go_to(format!(
			"/playlist?title={}&videos=[{{{}}}]",
			playlist.get_value().title,
			playlist
				.get_value()
				.videos
				.into_iter()
				.map(|video| video.id)
				.collect::<Vec<String>>()
				.join(",")
		));
	};

	view! {
		<div class="flex flex-col px-2 mt-3 space-y-3 w-full cursor-text">
			<h1 class="font-sans text-base font-semibold line-clamp-2">
				{playlist.get_value().title}
			</h1>
			<div class="flex flex-row flex-wrap gap-1 items-center text-sm font-normal">
				<h2
					on:click=go_to_local_playlist_page
					class="cursor-pointer text-primary"
				>
					{author}
				</h2>
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
pub fn Thumbnail(
	id: String,
	url: String,
	playlist: StoredValue<LocalPlaylist>,
) -> impl IntoView {
	let img_loaded = create_rw_signal(false);
	let image_classes = move || {
		if img_loaded.get() {
			"w-full aspect-video object-center object-cover bg-neutral rounded-xl".to_string()
		} else {
			"animate-pulse w-full aspect-video bg-neutral rounded-xl"
				.to_string()
		}
	};

	let go_to_local_playlist_page = move |_| {
		go_to(format!(
			"/playlist?title={}&videos=[{{{}}}]",
			playlist.get_value().title,
			playlist
				.get_value()
				.videos
				.into_iter()
				.map(|video| video.id)
				.collect::<Vec<String>>()
				.join(",")
		));
	};

	view! {
		<div
			on:click=go_to_local_playlist_page
			class="overflow-hidden w-full max-w-full rounded-xl"
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
