use invidious::{
	Channel, ChannelLivestreams, ChannelPlaylists, ChannelShorts,
	ChannelVideos, Comment, Comments, CommonPlaylist, CommonVideo,
};
use leptos::{
	expect_context, Action, Resource, RwSignal, SignalGet, SignalSet,
	SignalUpdate,
};
use leptos_router::create_query_signal;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelResourceArgs {
	server: String,
	locale: RustyTubeLocale,
	channel_id: String,
}

impl ChannelResourceArgs {
	pub fn new() -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			channel_id: create_query_signal("id").0.get().unwrap_or_default(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct ChannelResource {
	pub resource:
		Resource<ChannelResourceArgs, Result<Channel, RustyTubeError>>,
}

impl ChannelResource {
	pub fn initialise() -> Self {
		Self {
			resource: Resource::local(ChannelResourceArgs::new, move |args| {
				fetch_channel(args)
			}),
		}
	}
}

async fn fetch_channel(
	args: ChannelResourceArgs,
) -> Result<Channel, RustyTubeError> {
	Channel::fetch_channel(
		&args.server,
		&args.channel_id,
		args.locale.to_invidious_lang(),
	)
	.await
}

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelVideosResourceArgs {
	server: String,
	locale: RustyTubeLocale,
	channel_id: String,
}

impl ChannelVideosResourceArgs {
	pub fn new() -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			channel_id: create_query_signal("id").0.get().unwrap_or_default(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct ChannelVideosResource {
	pub resource: Resource<
		ChannelVideosResourceArgs,
		Result<ChannelVideos, RustyTubeError>,
	>,
}

impl ChannelVideosResource {
	pub fn initialise() -> Self {
		Self {
			resource: Resource::local(
				ChannelVideosResourceArgs::new,
				fetch_channel_videos,
			),
		}
	}
}

async fn fetch_channel_videos(
	args: ChannelVideosResourceArgs,
) -> Result<ChannelVideos, RustyTubeError> {
	Channel::fetch_channel_videos(
		&args.server,
		&args.channel_id,
		None,
		args.locale.to_invidious_lang(),
	)
	.await
}

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelVideosActionArgs {
	server: String,
	locale: RustyTubeLocale,
	channel_id: String,
	channel_videos_vec: RwSignal<Vec<CommonVideo>>,
	continuation: RwSignal<Option<String>>,
}

impl ChannelVideosActionArgs {
	pub fn get(
		channel_videos_vec: RwSignal<Vec<CommonVideo>>,
		continuation: RwSignal<Option<String>>,
	) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			channel_id: create_query_signal("id").0.get().unwrap_or_default(),
			channel_videos_vec,
			continuation,
		}
	}
}

#[derive(Clone, Copy)]
pub struct ChannelVideosAction {
	pub action: Action<ChannelVideosActionArgs, Result<(), RustyTubeError>>,
}

impl ChannelVideosAction {
	pub fn new() -> Self {
		Self {
			action: Action::new(|args: &ChannelVideosActionArgs| {
				fetch_more_channel_videos(args.clone())
			}),
		}
	}
}

async fn fetch_more_channel_videos(
	args: ChannelVideosActionArgs,
) -> Result<(), RustyTubeError> {
	let mut channel_videos = Channel::fetch_channel_videos(
		&args.server,
		&args.channel_id,
		args.continuation.get().as_deref(),
		args.locale.to_invidious_lang(),
	)
	.await?;
	args.channel_videos_vec
		.update(|videos| videos.append(&mut channel_videos.videos));
	args.continuation.set(channel_videos.continuation);
	Ok(())
}

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelShortsResourceArgs {
	server: String,
	locale: RustyTubeLocale,
	channel_id: String,
}

impl ChannelShortsResourceArgs {
	pub fn new() -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			channel_id: create_query_signal("id").0.get().unwrap_or_default(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct ChannelShortsResource {
	pub resource: Resource<
		ChannelShortsResourceArgs,
		Result<ChannelShorts, RustyTubeError>,
	>,
}

impl ChannelShortsResource {
	pub fn initialise() -> Self {
		Self {
			resource: Resource::local(
				ChannelShortsResourceArgs::new,
				fetch_channel_shorts,
			),
		}
	}
}

async fn fetch_channel_shorts(
	args: ChannelShortsResourceArgs,
) -> Result<ChannelShorts, RustyTubeError> {
	Channel::fetch_channel_shorts(
		&args.server,
		&args.channel_id,
		None,
		args.locale.to_invidious_lang(),
	)
	.await
}

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelShortsActionArgs {
	server: String,
	locale: RustyTubeLocale,
	channel_id: String,
	channel_shorts_vec: RwSignal<Vec<CommonVideo>>,
	continuation: RwSignal<Option<String>>,
}

impl ChannelShortsActionArgs {
	pub fn get(
		channel_shorts_vec: RwSignal<Vec<CommonVideo>>,
		continuation: RwSignal<Option<String>>,
	) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			channel_id: create_query_signal("id").0.get().unwrap_or_default(),
			channel_shorts_vec,
			continuation,
		}
	}
}

#[derive(Clone, Copy)]
pub struct ChannelShortsAction {
	pub action: Action<ChannelShortsActionArgs, Result<(), RustyTubeError>>,
}

impl ChannelShortsAction {
	pub fn new() -> Self {
		Self {
			action: Action::new(|args: &ChannelShortsActionArgs| {
				fetch_more_channel_shorts(args.clone())
			}),
		}
	}
}

async fn fetch_more_channel_shorts(
	args: ChannelShortsActionArgs,
) -> Result<(), RustyTubeError> {
	let mut channel_shorts = Channel::fetch_channel_shorts(
		&args.server,
		&args.channel_id,
		args.continuation.get().as_deref(),
		args.locale.to_invidious_lang(),
	)
	.await?;
	args.channel_shorts_vec
		.update(|shorts| shorts.append(&mut channel_shorts.shorts));
	args.continuation.set(channel_shorts.continuation);
	Ok(())
}

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelLivestreamsResourceArgs {
	server: String,
	locale: RustyTubeLocale,
	channel_id: String,
}

impl ChannelLivestreamsResourceArgs {
	pub fn new() -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			channel_id: create_query_signal("id").0.get().unwrap_or_default(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct ChannelLivestreamsResource {
	pub resource: Resource<
		ChannelLivestreamsResourceArgs,
		Result<ChannelLivestreams, RustyTubeError>,
	>,
}

impl ChannelLivestreamsResource {
	pub fn initialise() -> Self {
		Self {
			resource: Resource::local(
				ChannelLivestreamsResourceArgs::new,
				fetch_channel_livestreams,
			),
		}
	}
}

async fn fetch_channel_livestreams(
	args: ChannelLivestreamsResourceArgs,
) -> Result<ChannelLivestreams, RustyTubeError> {
	Channel::fetch_channel_livestreams(
		&args.server,
		&args.channel_id,
		None,
		args.locale.to_invidious_lang(),
	)
	.await
}

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelLivestreamsActionArgs {
	server: String,
	locale: RustyTubeLocale,
	channel_id: String,
	channel_livestreams_vec: RwSignal<Vec<CommonVideo>>,
	continuation: RwSignal<Option<String>>,
}

impl ChannelLivestreamsActionArgs {
	pub fn get(
		channel_livestreams_vec: RwSignal<Vec<CommonVideo>>,
		continuation: RwSignal<Option<String>>,
	) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			channel_id: create_query_signal("id").0.get().unwrap_or_default(),
			channel_livestreams_vec,
			continuation,
		}
	}
}

#[derive(Clone, Copy)]
pub struct ChannelLivestreamsAction {
	pub action:
		Action<ChannelLivestreamsActionArgs, Result<(), RustyTubeError>>,
}

impl ChannelLivestreamsAction {
	pub fn new() -> Self {
		Self {
			action: Action::new(|args: &ChannelLivestreamsActionArgs| {
				fetch_more_channel_livestreams(args.clone())
			}),
		}
	}
}

async fn fetch_more_channel_livestreams(
	args: ChannelLivestreamsActionArgs,
) -> Result<(), RustyTubeError> {
	let mut channel_livestreams = Channel::fetch_channel_livestreams(
		&args.server,
		&args.channel_id,
		args.continuation.get().as_deref(),
		args.locale.to_invidious_lang(),
	)
	.await?;
	args.channel_livestreams_vec.update(|livestreams| {
		livestreams.append(&mut channel_livestreams.livestreams);
	});
	args.continuation.set(channel_livestreams.continuation);
	Ok(())
}

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelPlaylistsResourceArgs {
	server: String,
	locale: RustyTubeLocale,
	channel_id: String,
}

impl ChannelPlaylistsResourceArgs {
	pub fn new() -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			channel_id: create_query_signal("id").0.get().unwrap_or_default(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct ChannelPlaylistsResource {
	pub resource: Resource<
		ChannelPlaylistsResourceArgs,
		Result<ChannelPlaylists, RustyTubeError>,
	>,
}

impl ChannelPlaylistsResource {
	pub fn initialise() -> Self {
		Self {
			resource: Resource::local(
				ChannelPlaylistsResourceArgs::new,
				fetch_channel_playlists,
			),
		}
	}
}

async fn fetch_channel_playlists(
	args: ChannelPlaylistsResourceArgs,
) -> Result<ChannelPlaylists, RustyTubeError> {
	Channel::fetch_channel_playlists(
		&args.server,
		&args.channel_id,
		None,
		args.locale.to_invidious_lang(),
	)
	.await
}

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelPlaylistsActionArgs {
	server: String,
	locale: RustyTubeLocale,
	channel_id: String,
	channel_playlists_vec: RwSignal<Vec<CommonPlaylist>>,
	continuation: RwSignal<Option<String>>,
}

impl ChannelPlaylistsActionArgs {
	pub fn get(
		channel_playlists_vec: RwSignal<Vec<CommonPlaylist>>,
		continuation: RwSignal<Option<String>>,
	) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			channel_id: create_query_signal("id").0.get().unwrap_or_default(),
			channel_playlists_vec,
			continuation,
		}
	}
}

#[derive(Clone, Copy)]
pub struct ChannelPlaylistsAction {
	pub action: Action<ChannelPlaylistsActionArgs, Result<(), RustyTubeError>>,
}

impl ChannelPlaylistsAction {
	pub fn new() -> Self {
		Self {
			action: Action::new(|args: &ChannelPlaylistsActionArgs| {
				fetch_more_channel_playlists(args.clone())
			}),
		}
	}
}

async fn fetch_more_channel_playlists(
	args: ChannelPlaylistsActionArgs,
) -> Result<(), RustyTubeError> {
	let mut channel_playlists = Channel::fetch_channel_playlists(
		&args.server,
		&args.channel_id,
		args.continuation.get().as_deref(),
		args.locale.to_invidious_lang(),
	)
	.await?;
	args.channel_playlists_vec
		.update(|playlists| playlists.append(&mut channel_playlists.playlists));
	args.continuation.set(channel_playlists.continuation);
	Ok(())
}
