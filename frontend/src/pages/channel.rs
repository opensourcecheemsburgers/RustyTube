use invidious::{
	Channel, ChannelLivestreams, ChannelPlaylists, ChannelShorts, ChannelVideos, CommonPlaylist,
	CommonVideo,
};
use leptos::*;
use leptos_router::create_query_signal;
use rustytube_error::RustyTubeError;

use crate::{
	components::{FerrisError, PlaceholderCardArray, PlaylistPreviewCard, VideoPreviewCard},
	contexts::{ServerCtx, SubscriptionsCtx},
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
	let server = expect_context::<ServerCtx>().0 .0;
	let id_query: Memo<Option<String>> = create_query_signal("id").0;

	let channel = create_resource(
		move || (server.get(), id_query.get().unwrap_or_default()),
		|(server, id)| async move { Channel::fetch_channel(&server, &id).await },
	);

	view! {
		<Suspense fallback=move || {
			view! { <ChannelSectionPlaceholder/> }
		}>
			{move || {
				channel
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
		<div class="w-full flex justify-center mt-4">
			<div class="w-[90%] flex flex-col gap-y-8">
				<Header/>
				<ContentCategoryButtons/>
				<Content/>
			</div>
		</div>
	}
}

#[component]
fn Header() -> impl IntoView {
	let channel = expect_context::<Channel>();

	view! {
		<div class="flex flex-col space-y-8 self-center">
			<Banner/>
			<div class="flex flex-row items-center space-x-2">
				<ChannelAvatar/>
				<div class="flex h-16 flex-col justify-around">
					<h1 class="font-semibold text-lg">{channel.name}</h1>
					<SubscribeBtn/>
				</div>
			</div>
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
	let image_classes = move || match img_loaded.get() {
		true => "h-16 w-16 rounded-full".to_string(),
		false => "h-16 w-16 animate-pulse rounded-full bg-neutral".to_string(),
	};

	view! { <img on:load=move |_| img_loaded.set(true) src=url class=image_classes/> }
}

#[component]
fn Banner() -> impl IntoView {
	let channel = expect_context::<Channel>();
	let url = channel.banners.first().map(|banner| banner.url.clone());
	let img_loaded = create_rw_signal(false);
	let image_classes = move || match img_loaded.get() {
		true => "w-full object-center object-cover bg-neutral rounded-xl".to_string(),
		false => "w-full h-full rounded-xl bg-neutral".to_string(),
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
			.find(|sub| sub.id.eq_ignore_ascii_case(&channel_id.get_value()))
			.is_some()
	};
	let is_subscribed = RwSignal::new(check_subscribed());

	let add_sub = create_action(|args: &(String, String, SubscriptionsCtx)| {
		let name = args.0.clone();
		let id = args.1.clone();
		let subscriptions_ctx = args.2.clone();
		async move { subscriptions_ctx.add_subscription(&id, &name).await }
	});

	let remove_sub = create_action(|args: &(String, SubscriptionsCtx)| {
		let id = args.0.clone();
		let subscriptions_ctx = args.1.clone();
		async move { subscriptions_ctx.remove_subscription(&id).await }
	});

	let btn_text = move || match is_subscribed.get() {
		true => "Subscribed",
		false => "Subscribe",
	};

	let on_click = move |_| match is_subscribed.get() {
		true => remove_sub.dispatch((channel_id.get_value(), subscriptions_ctx)),
		false => {
			add_sub.dispatch((channel_name.get_value(), channel_id.get_value(), subscriptions_ctx))
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
	let content_category = expect_context::<RwSignal<ContentCategory>>();

	view! {
		<div class="flex flex-row gap-x-3">
			<button
				on:click=move |_| content_category.set(ContentCategory::Videos)
				class="btn btn-outline btn-sm rounded-lg font-normal normal-case"
			>
				Videos
			</button>
			<button
				on:click=move |_| content_category.set(ContentCategory::Shorts)
				class="btn btn-outline btn-sm rounded-lg font-normal normal-case"
			>
				Shorts
			</button>
			<button
				on:click=move |_| content_category.set(ContentCategory::Livestreams)
				class="btn btn-outline btn-sm rounded-lg font-normal normal-case"
			>
				Livestreams
			</button>
			<button
				on:click=move |_| content_category.set(ContentCategory::Playlists)
				class="btn btn-outline btn-sm rounded-lg font-normal normal-case"
			>
				Playlists
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
fn ContentContainer(children: Children) -> impl IntoView {
	view! {
		<div class="-ml-4 flex flex-row flex-wrap gap-y-12 pb-12 overflow-y-hidden hover:overflow-y-auto scroll-smooth">
			{children()}
		</div>
	}
}

#[component]
fn Videos() -> impl IntoView {
	let server = expect_context::<ServerCtx>().0 .0;
	let id_query: Memo<Option<String>> = create_query_signal("id").0;

	let videos = create_resource(
		move || (server.get(), id_query.get().unwrap_or_default()),
		|(server, id)| async move { Channel::fetch_channel_videos(&server, &id, None).await },
	);

	view! {
		<Suspense fallback=move || {
			view! { <PlaceholderCardArray/> }
		}>
			{move || {
				videos
					.get()
					.map(|videos| match videos {
						Ok(videos) => view! { <VideosInner videos=videos/> },
						Err(err) => view! { <FerrisError error=err/> }.into_view(),
					})
			}}

		</Suspense>
	}
}

#[component]
fn VideosInner(videos: ChannelVideos) -> impl IntoView {
	let server = expect_context::<ServerCtx>().0 .0;
	let id = RwSignal::new(create_query_signal::<String>("id").0.get().unwrap_or_default());
	let continuation = RwSignal::new(videos.continuation);
	let videos_vec = RwSignal::new(videos.videos);
	let fetch_more_videos = create_action(|args: &VideosFetchArgs| fetch_more_videos(*args));

	let video_fetch_args =
		VideosFetchArgs::new(videos_vec, server, id, continuation, fetch_more_videos);

	let videos_view = move || {
		videos_vec.get().into_iter().map(|video| view! { <VideoPreviewCard video/> }).collect_view()
	};

	let load_more_videos = move |_| fetch_more_videos.dispatch(video_fetch_args);
	let fetch_more_btn = move || match continuation.get() {
		None => ().into_view(),
		Some(_) => view! {
			<button class="btn btn-primary btn-outline btn-sm" on:click=load_more_videos>
				{"Load more"}
			</button>
		}
		.into_view(),
	};

	view! {
		<div class="pb-12">
			<ContentContainer>{videos_view}</ContentContainer>
			{fetch_more_btn}
		</div>
	}
}

async fn fetch_more_videos(args: VideosFetchArgs) -> Result<(), RustyTubeError> {
	let mut channel_videos = Channel::fetch_channel_videos(
		&args.server.get(),
		&args.id.get(),
		args.continuation.get().as_deref(),
	)
	.await?;
	args.videos_vec.update(|videos| videos.append(&mut channel_videos.videos));
	args.continuation.set(channel_videos.continuation);
	Ok(())
}

#[component]
fn Shorts() -> impl IntoView {
	let server = expect_context::<ServerCtx>().0 .0;
	let id_query: Memo<Option<String>> = create_query_signal("id").0;

	let shorts = create_resource(
		move || (server.get(), id_query.get().unwrap_or_default()),
		|(server, id)| async move { Channel::fetch_channel_shorts(&server, &id, None).await },
	);

	view! {
		<Suspense fallback=move || {
			view! { <PlaceholderCardArray/> }
		}>
			{move || {
				shorts
					.get()
					.map(|shorts| match shorts {
						Ok(shorts) => view! { <ShortsInner shorts=shorts/> },
						Err(err) => view! { <FerrisError error=err/> }.into_view(),
					})
			}}

		</Suspense>
	}
}

#[component]
fn ShortsInner(shorts: ChannelShorts) -> impl IntoView {
	let server = expect_context::<ServerCtx>().0 .0;
	let id = RwSignal::new(create_query_signal::<String>("id").0.get().unwrap_or_default());
	let continuation = RwSignal::new(shorts.continuation);
	let shorts_vec = RwSignal::new(shorts.shorts);
	let fetch_more_shorts = create_action(|args: &ShortsFetchArgs| fetch_more_shorts(*args));

	let shorts_fetch_args =
		ShortsFetchArgs::new(shorts_vec, server, id, continuation, fetch_more_shorts);

	let shorts_view = move || {
		shorts_vec
			.get()
			.into_iter()
			.map(|short| view! { <VideoPreviewCard video=short/> })
			.collect_view()
	};

	let load_more_shorts = move |_| fetch_more_shorts.dispatch(shorts_fetch_args);
	let fetch_more_btn = move || match continuation.get() {
		None => ().into_view(),
		Some(_) => view! {
			<button class="btn btn-primary btn-outline btn-sm" on:click=load_more_shorts>
				{"Load more"}
			</button>
		}
		.into_view(),
	};

	view! {
		<div class="pb-12">
			<ContentContainer>{shorts_view}</ContentContainer>
			{fetch_more_btn}
		</div>
	}
}

async fn fetch_more_shorts(args: ShortsFetchArgs) -> Result<(), RustyTubeError> {
	let mut channel_shorts = Channel::fetch_channel_shorts(
		&args.server.get(),
		&args.id.get(),
		args.continuation.get().as_deref(),
	)
	.await?;
	args.shorts_vec.update(|shorts| shorts.append(&mut channel_shorts.shorts));
	args.continuation.set(channel_shorts.continuation);
	Ok(())
}

#[component]
fn Livestreams() -> impl IntoView {
	let server = expect_context::<ServerCtx>().0 .0;
	let id_query: Memo<Option<String>> = create_query_signal("id").0;

	let livestreams = create_resource(
		move || (server.get(), id_query.get().unwrap_or_default()),
		|(server, id)| async move { Channel::fetch_channel_livestreams(&server, &id, None).await },
	);

	view! {
		<Suspense fallback=move || {
			view! { <PlaceholderCardArray/> }
		}>
			{move || {
				livestreams
					.get()
					.map(|videos| match videos {
						Ok(channel_livestreams) => {
							view! { <LivestreamsInner livestreams=channel_livestreams/> }
						}
						Err(err) => view! { <FerrisError error=err/> },
					})
			}}

		</Suspense>
	}
}

#[component]
fn LivestreamsInner(livestreams: ChannelLivestreams) -> impl IntoView {
	let server = expect_context::<ServerCtx>().0 .0;
	let id = RwSignal::new(create_query_signal::<String>("id").0.get().unwrap_or_default());
	let continuation = RwSignal::new(livestreams.continuation);
	let livestreams_vec = RwSignal::new(livestreams.livestreams);
	let fetch_more_livestreams =
		create_action(|args: &LivestreamsFetchArgs| fetch_more_livestreams(*args));

	let livestream_fetch_args = LivestreamsFetchArgs::new(
		livestreams_vec,
		server,
		id,
		continuation,
		fetch_more_livestreams,
	);

	let livestreams_view = move || {
		livestreams_vec
			.get()
			.into_iter()
			.map(|livestream| view! { <VideoPreviewCard video=livestream/> })
			.collect_view()
	};

	let load_more_livestreams = move |_| fetch_more_livestreams.dispatch(livestream_fetch_args);
	let fetch_more_btn = move || match continuation.get() {
		None => ().into_view(),
		Some(_) => view! {
			<button class="btn btn-primary btn-outline btn-sm" on:click=load_more_livestreams>
				{"Load more"}
			</button>
		}
		.into_view(),
	};

	view! {
		<div class="pb-12">
			<ContentContainer>{livestreams_view}</ContentContainer>
			{fetch_more_btn}
		</div>
	}
}

async fn fetch_more_livestreams(args: LivestreamsFetchArgs) -> Result<(), RustyTubeError> {
	let mut channel_livestreams = Channel::fetch_channel_livestreams(
		&args.server.get(),
		&args.id.get(),
		args.continuation.get().as_deref(),
	)
	.await?;
	args.livestreams_vec.update(|shorts| shorts.append(&mut channel_livestreams.livestreams));
	args.continuation.set(channel_livestreams.continuation);
	Ok(())
}

#[component]
fn Playlists() -> impl IntoView {
	let server = expect_context::<ServerCtx>().0 .0;
	let id_query: Memo<Option<String>> = create_query_signal("id").0;

	let playlists = create_resource(
		move || (server.get(), id_query.get().unwrap_or_default()),
		|(server, id)| async move { Channel::fetch_channel_playlists(&server, &id, None).await },
	);

	view! {
		<Suspense fallback=move || {
			view! { <PlaceholderCardArray/> }
		}>
			{move || {
				playlists
					.get()
					.map(|playlists_result| match playlists_result {
						Ok(playlists) => view! { <PlaylistsInner playlists=playlists/> },
						Err(err) => view! { <FerrisError error=err/> },
					})
			}}

		</Suspense>
	}
}

#[component]
fn PlaylistsInner(playlists: ChannelPlaylists) -> impl IntoView {
	let server = expect_context::<ServerCtx>().0 .0;
	let id = RwSignal::new(create_query_signal::<String>("id").0.get().unwrap_or_default());
	let continuation = RwSignal::new(playlists.continuation);
	let playlists_vec = RwSignal::new(playlists.playlists);
	let fetch_more_playlists =
		create_action(|args: &PlaylistsFetchArgs| fetch_more_playlists(*args));

	let playlists_fetch_args =
		PlaylistsFetchArgs::new(playlists_vec, server, id, continuation, fetch_more_playlists);

	let playlists_view = move || {
		playlists_vec
			.get()
			.into_iter()
			.map(|playlist| view! { <PlaylistPreviewCard playlist=playlist/> })
			.collect_view()
	};

	let load_more_playlists = move |_| fetch_more_playlists.dispatch(playlists_fetch_args);
	let fetch_more_btn = move || match continuation.get() {
		None => ().into_view(),
		Some(_) => view! {
			<button class="btn btn-primary btn-outline btn-sm" on:click=load_more_playlists>
				{"Load more"}
			</button>
		}
		.into_view(),
	};

	view! {
		<div class="pb-12">
			<ContentContainer>{playlists_view}</ContentContainer>
			{fetch_more_btn}
		</div>
	}
}

async fn fetch_more_playlists(args: PlaylistsFetchArgs) -> Result<(), RustyTubeError> {
	let mut channel_playlists = Channel::fetch_channel_playlists(
		&args.server.get(),
		&args.id.get(),
		args.continuation.get().as_deref(),
	)
	.await?;
	args.playlists_vec.update(|shorts| shorts.append(&mut channel_playlists.playlists));
	args.continuation.set(channel_playlists.continuation);
	Ok(())
}

#[component]
fn ChannelSectionPlaceholder() -> impl IntoView {
	view! {
		<div class="flex flex-col space-y-8 self-center">
			<div class="h-36 w-full rounded-xl bg-neutral"></div>

			<div class="flex flex-row justify-between">
				<div class="flex flex-row items-center space-x-2">
					<div class="h-16 w-16 rounded-full bg-neutral"></div>
					<div class="flex h-16 flex-col justify-around">
						<div class="h-4 w-32 rounded bg-neutral"></div>
						<div class="h-4 w-32 rounded bg-neutral"></div>
					</div>
				</div>

				<div class="flex flex-row space-x-3">
					<div class="h-16 w-16 rounded-full border-2 border-neutral"></div>
				</div>
			</div>

			<div class="flex flex-row gap-x-3">
				<div class="h-8 w-20 rounded-lg bg-neutral"></div>
				<div class="h-8 w-20 rounded-lg bg-neutral"></div>
				<div class="h-8 w-32 rounded-lg bg-neutral"></div>
				<div class="h-8 w-24 rounded-lg bg-neutral"></div>
			</div>
		</div>
	}
}

#[derive(Clone, Copy)]
struct VideosFetchArgs {
	pub videos_vec: RwSignal<Vec<CommonVideo>>,
	pub server: Signal<String>,
	pub id: RwSignal<String>,
	pub continuation: RwSignal<Option<String>>,
	pub fetch_more_videos: Action<Self, Result<(), RustyTubeError>>,
}

impl VideosFetchArgs {
	fn new(
		videos_vec: RwSignal<Vec<CommonVideo>>,
		server: Signal<String>,
		id: RwSignal<String>,
		continuation: RwSignal<Option<String>>,
		fetch_more_videos: Action<Self, Result<(), RustyTubeError>>,
	) -> Self {
		Self { videos_vec, server, id, continuation, fetch_more_videos }
	}
}

#[derive(Clone, Copy)]
struct ShortsFetchArgs {
	pub shorts_vec: RwSignal<Vec<CommonVideo>>,
	pub server: Signal<String>,
	pub id: RwSignal<String>,
	pub continuation: RwSignal<Option<String>>,
	pub fetch_more_shorts: Action<Self, Result<(), RustyTubeError>>,
}

impl ShortsFetchArgs {
	fn new(
		shorts_vec: RwSignal<Vec<CommonVideo>>,
		server: Signal<String>,
		id: RwSignal<String>,
		continuation: RwSignal<Option<String>>,
		fetch_more_shorts: Action<Self, Result<(), RustyTubeError>>,
	) -> Self {
		Self { shorts_vec, server, id, continuation, fetch_more_shorts }
	}
}

#[derive(Clone, Copy)]
struct LivestreamsFetchArgs {
	pub livestreams_vec: RwSignal<Vec<CommonVideo>>,
	pub server: Signal<String>,
	pub id: RwSignal<String>,
	pub continuation: RwSignal<Option<String>>,
	pub fetch_more_livestreams: Action<Self, Result<(), RustyTubeError>>,
}

impl LivestreamsFetchArgs {
	fn new(
		livestreams_vec: RwSignal<Vec<CommonVideo>>,
		server: Signal<String>,
		id: RwSignal<String>,
		continuation: RwSignal<Option<String>>,
		fetch_more_livestreams: Action<Self, Result<(), RustyTubeError>>,
	) -> Self {
		Self { livestreams_vec, server, id, continuation, fetch_more_livestreams }
	}
}

#[derive(Clone, Copy)]
struct PlaylistsFetchArgs {
	pub playlists_vec: RwSignal<Vec<CommonPlaylist>>,
	pub server: Signal<String>,
	pub id: RwSignal<String>,
	pub continuation: RwSignal<Option<String>>,
	pub fetch_more_playlists: Action<Self, Result<(), RustyTubeError>>,
}

impl PlaylistsFetchArgs {
	fn new(
		playlists_vec: RwSignal<Vec<CommonPlaylist>>,
		server: Signal<String>,
		id: RwSignal<String>,
		continuation: RwSignal<Option<String>>,
		fetch_more_playlists: Action<Self, Result<(), RustyTubeError>>,
	) -> Self {
		Self { playlists_vec, server, id, continuation, fetch_more_playlists }
	}
}
