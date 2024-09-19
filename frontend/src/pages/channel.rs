use invidious::{
	Channel, ChannelLivestreams, ChannelPlaylists, ChannelShorts,
	ChannelVideos, CommonPlaylist, CommonVideo,
};
use leptos::*;
use num_format::ToFormattedString;

use crate::{
	components::{
		CardGrid, ChannelRoll, FerrisError, GridContainer,
		PlaceholderCardArray, PlaylistPreviewCard, VideoPreviewCard,
	},
	contexts::RegionConfigCtx,
	resources::{
		ChannelLivestreamsAction, ChannelLivestreamsActionArgs,
		ChannelLivestreamsResource, ChannelPlaylistsAction,
		ChannelPlaylistsActionArgs, ChannelPlaylistsResource, ChannelResource,
		ChannelShortsAction, ChannelShortsActionArgs, ChannelShortsResource,
		ChannelVideosAction, ChannelVideosActionArgs, ChannelVideosResource,
		SubscriptionsCtx,
	},
	utils::i18n,
};

#[derive(Clone)]
enum ContentCategory {
	Videos,
	Shorts,
	Livestreams,
	Playlists,
}

#[component]
pub fn ChannelPage() -> impl IntoView {
	let channel = ChannelResource::initialise();

	view! {
		<Suspense fallback=move || {
			view! { <ChannelSectionPlaceholder/> }
		}>
			{move || {
				channel
					.resource
					.get()
					.map(|channel_result| {
						match channel_result {
							Ok(channel) => view! { <ChannelPageInner channel=channel/> },
							Err(err) => view! { <FerrisError error=err/> },
						}
					})
			}}

		</Suspense>
	}
}

#[component]
fn ChannelPageInner(channel: Channel) -> impl IntoView {
	provide_context(channel);
	provide_context(RwSignal::new(ContentCategory::Videos));

	view! {
		<GridContainer>
			<Header/>
			<ContentCategoryButtons/>
			<Content/>
		</GridContainer>
	}
}

#[component]
fn Header() -> impl IntoView {
	let locale = expect_context::<RegionConfigCtx>().locale_slice.0;
	let channel = expect_context::<Channel>();

	let sub_count = move || {
		channel.subscribers.to_formatted_string(&locale.get().to_num_fmt())
	};

	view! {
		<div class="flex flex-col self-center space-y-8">
			<Banner/>
			<ChannelRoll
				channel=channel.name
				channel_id=channel.id
				sub_count=sub_count()
				image_url=channel
					.thumbnails
					.first()
					.map_or(String::new(), |thumb| thumb.url.clone())
			/>
		</div>
	}
}

#[component]
fn ChannelInfo() -> impl IntoView {}

#[component]
fn ChannelAvatar() -> impl IntoView {
	let channel = expect_context::<Channel>();
	let url = channel.thumbnails.first().map(|channel| channel.url.clone());
	let img_loaded = create_rw_signal(false);
	let image_classes = move || {
		if img_loaded.get() {
			"h-16 w-16 rounded-full".to_string()
		} else {
			"h-16 w-16 animate-pulse rounded-full bg-neutral".to_string()
		}
	};

	view! {
		<img on:load=move |_| img_loaded.set(true) src=url class=image_classes/>
	}
}

#[component]
fn Banner() -> impl IntoView {
	let channel = expect_context::<Channel>();
	let url = channel.banners.first().map(|banner| banner.url.clone());
	let img_loaded = create_rw_signal(false);
	let image_classes = move || {
		if img_loaded.get() {
			"w-full object-center object-cover bg-neutral rounded-xl"
				.to_string()
		} else {
			"w-full h-full rounded-xl bg-neutral".to_string()
		}
	};

	view! {
		<div class="w-full rounded-xl bg-neutral">
			<img
				decoding="async"
				on:load=move |_| img_loaded.set(true)
				src=url
				class=image_classes
			/>
		</div>
	}
}

#[component]
fn SubscribeBtn() -> impl IntoView {
	let channel = expect_context::<Channel>();

	let channel_name = StoredValue::new(channel.name);
	let channel_id = StoredValue::new(channel.id);

	let subscriptions_ctx = expect_context::<SubscriptionsCtx>();

	let check_subscribed = move || {
		subscriptions_ctx
			.0
			.get()
			.channels
			.into_iter()
			.any(|sub| sub.id.eq_ignore_ascii_case(&channel_id.get_value()))
	};
	let is_subscribed = RwSignal::new(check_subscribed());

	let add_sub = create_action(|args: &(String, String, SubscriptionsCtx)| {
		let name = args.0.clone();
		let id = args.1.clone();
		let subscriptions_ctx = args.2;
		async move { subscriptions_ctx.add_subscription(&id, &name).await }
	});

	let remove_sub = create_action(|args: &(String, SubscriptionsCtx)| {
		let id = args.0.clone();
		let subscriptions_ctx = args.1;
		async move { subscriptions_ctx.remove_subscription(&id).await }
	});

	let btn_text = move || {
		if is_subscribed.get() {
			"Subscribed"
		} else {
			"Subscribe"
		}
	};

	let on_click = move |_| {
		if is_subscribed.get() {
			remove_sub.dispatch((channel_id.get_value(), subscriptions_ctx));
		} else {
			add_sub.dispatch((
				channel_name.get_value(),
				channel_id.get_value(),
				subscriptions_ctx,
			));
		}
	};

	view! {
		<button on:click=on_click class="btn btn-primary btn-xs">
			{btn_text}
		</button>
	}
}

#[component]
fn ContentCategoryButtons() -> impl IntoView {
	let category = expect_context::<RwSignal<ContentCategory>>();

	view! {
		<div class="flex flex-row gap-x-3">
			<button
				on:click=move |_| category.set(ContentCategory::Videos)
				class="font-normal normal-case rounded-lg btn btn-outline btn-xs sm:btn-sm"
			>
				{i18n("channel.videos")}
			</button>
			<button
				on:click=move |_| category.set(ContentCategory::Shorts)
				class="font-normal normal-case rounded-lg btn btn-outline btn-xs sm:btn-sm"
			>
				{i18n("channel.shorts")}
			</button>
			<button
				on:click=move |_| category.set(ContentCategory::Livestreams)
				class="font-normal normal-case rounded-lg btn btn-outline btn-xs sm:btn-sm"
			>
				{i18n("channel.livestreams")}
			</button>
			<button
				on:click=move |_| category.set(ContentCategory::Playlists)
				class="font-normal normal-case rounded-lg btn btn-outline btn-xs sm:btn-sm"
			>
				{i18n("channel.playlists")}
			</button>
		</div>
	}
}

#[component]
fn Content() -> impl IntoView {
	let content_category = expect_context::<RwSignal<ContentCategory>>();

	move || match content_category.get() {
		ContentCategory::Videos => view! { <Videos/> },
		ContentCategory::Shorts => view! { <Shorts/> },
		ContentCategory::Livestreams => view! { <Livestreams/> },
		ContentCategory::Playlists => view! { <Playlists/> },
	}
}

#[component]
fn Videos() -> impl IntoView {
	let videos = ChannelVideosResource::initialise();

	view! {
		<Suspense fallback=move || {
			view! { <PlaceholderCardArray/> }
		}>
			{move || {
				videos
					.resource
					.get()
					.map(|videos| match videos {
						Ok(videos) => view! { <VideosInner channel_videos=videos/> },
						Err(err) => view! { <FerrisError error=err/> }.into_view(),
					})
			}}

		</Suspense>
	}
}

#[component]
fn VideosInner(channel_videos: ChannelVideos) -> impl IntoView {
	let videos_vec = RwSignal::new(channel_videos.videos);
	let continuation = RwSignal::new(channel_videos.continuation);
	let channel_videos_action = ChannelVideosAction::new();

	view! {
		<CardGrid>
			<For
				each=move || videos_vec.get()
				key=|video: &CommonVideo| video.id.clone()
				let:video
			>
				<VideoPreviewCard video=video/>
			</For>
		</CardGrid>

		<Show when=move || continuation.get().is_some()>
			<button
				class="btn btn-primary btn-outline btn-sm"
				on:click=move |_| {
					channel_videos_action
						.action
						.dispatch(
							ChannelVideosActionArgs::get(videos_vec, continuation),
						);
				}
			>

				{i18n("general.load_more")}
			</button>
		</Show>
	}
}

#[component]
fn Shorts() -> impl IntoView {
	let shorts = ChannelShortsResource::initialise();

	view! {
		<Suspense fallback=move || {
			view! { <PlaceholderCardArray/> }
		}>
			{move || {
				shorts
					.resource
					.get()
					.map(|shorts| match shorts {
						Ok(shorts) => view! { <ShortsInner channel_shorts=shorts/> },
						Err(err) => view! { <FerrisError error=err/> }.into_view(),
					})
			}}

		</Suspense>
	}
}

#[component]
fn ShortsInner(channel_shorts: ChannelShorts) -> impl IntoView {
	let shorts_vec = RwSignal::new(channel_shorts.shorts);
	let continuation = RwSignal::new(channel_shorts.continuation);
	let channel_shorts_action = ChannelShortsAction::new();

	view! {
		<CardGrid>
			<For
				each=move || shorts_vec.get()
				key=|video: &CommonVideo| video.id.clone()
				let:video
			>
				<VideoPreviewCard video=video/>
			</For>
		</CardGrid>

		<Show when=move || continuation.get().is_some()>
			<button
				class="btn btn-primary btn-outline btn-sm"
				on:click=move |_| {
					channel_shorts_action
						.action
						.dispatch(
							ChannelShortsActionArgs::get(shorts_vec, continuation),
						);
				}
			>

				{i18n("general.load_more")}
			</button>
		</Show>
	}
}

#[component]
fn Livestreams() -> impl IntoView {
	let livestreams = ChannelLivestreamsResource::initialise();

	view! {
		<Suspense fallback=move || {
			view! { <PlaceholderCardArray/> }
		}>
			{move || {
				livestreams
					.resource
					.get()
					.map(|livestreams| match livestreams {
						Ok(livestreams) => {
							view! {
								<LivestreamsInner channel_livestreams=livestreams/>
							}
						}
						Err(err) => view! { <FerrisError error=err/> }.into_view(),
					})
			}}

		</Suspense>
	}
}

#[component]
fn LivestreamsInner(channel_livestreams: ChannelLivestreams) -> impl IntoView {
	let livestreams_vec = RwSignal::new(channel_livestreams.livestreams);
	let continuation = RwSignal::new(channel_livestreams.continuation);
	let channel_livestreams_action = ChannelLivestreamsAction::new();

	view! {
		<CardGrid>
			<For
				each=move || livestreams_vec.get()
				key=|video: &CommonVideo| video.id.clone()
				let:video
			>
				<VideoPreviewCard video=video/>
			</For>
		</CardGrid>

		<Show when=move || continuation.get().is_some()>
			<button
				class="btn btn-primary btn-outline btn-sm"
				on:click=move |_| {
					channel_livestreams_action
						.action
						.dispatch(
							ChannelLivestreamsActionArgs::get(
								livestreams_vec,
								continuation,
							),
						);
				}
			>

				{i18n("general.load_more")}
			</button>
		</Show>
	}
}

#[component]
fn Playlists() -> impl IntoView {
	let playlists = ChannelPlaylistsResource::initialise();

	view! {
		<Suspense fallback=move || {
			view! { <PlaceholderCardArray/> }
		}>
			{move || {
				playlists
					.resource
					.get()
					.map(|playlists| match playlists {
						Ok(playlists) => {
							view! { <PlaylistsInner channel_playlists=playlists/> }
						}
						Err(err) => view! { <FerrisError error=err/> }.into_view(),
					})
			}}

		</Suspense>
	}
}

#[component]
fn PlaylistsInner(channel_playlists: ChannelPlaylists) -> impl IntoView {
	let playlists_vec = RwSignal::new(channel_playlists.playlists);
	let continuation = RwSignal::new(channel_playlists.continuation);
	let channel_playlists_action = ChannelPlaylistsAction::new();

	view! {
		<CardGrid>
			<For
				each=move || playlists_vec.get()
				key=|playlist: &CommonPlaylist| playlist.id.clone()
				let:playlist
			>
				<PlaylistPreviewCard playlist=playlist/>
			</For>
		</CardGrid>

		<Show when=move || continuation.get().is_some()>
			<button
				class="btn btn-primary btn-outline btn-sm"
				on:click=move |_| {
					channel_playlists_action
						.action
						.dispatch(
							ChannelPlaylistsActionArgs::get(playlists_vec, continuation),
						);
				}
			>

				{i18n("general.load_more")}
			</button>
		</Show>
	}
}

#[component]
fn ChannelSectionPlaceholder() -> impl IntoView {
	view! {
		<div class="flex flex-col self-center space-y-8">
			<div class="w-full h-36 rounded-xl bg-neutral"></div>

			<div class="flex flex-row justify-between">
				<div class="flex flex-row gap-x-2 items-center">
					<div class="w-16 h-16 rounded-full bg-neutral"></div>
					<div class="flex flex-col justify-around h-16">
						<div class="w-32 h-4 rounded bg-neutral"></div>
						<div class="w-32 h-4 rounded bg-neutral"></div>
					</div>
				</div>

				<div class="flex flex-row gap-x-3">
					<div class="w-16 h-16 rounded-full border-2 border-neutral"></div>
				</div>
			</div>

			<div class="flex flex-row gap-x-3">
				<div class="w-20 h-8 rounded-lg bg-neutral"></div>
				<div class="w-20 h-8 rounded-lg bg-neutral"></div>
				<div class="w-32 h-8 rounded-lg bg-neutral"></div>
				<div class="w-24 h-8 rounded-lg bg-neutral"></div>
			</div>
		</div>
	}
}
