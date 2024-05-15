use invidious::Video;
use leptos::*;
use leptos_router::create_query_signal;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

static POPULAR_KEY: &'static str = "popular_videos";

#[derive(Clone, PartialEq)]
pub struct VideoResourceArgs {
	server: String,
	locale: RustyTubeLocale,
	id: String,
}

impl VideoResourceArgs {
	fn new() -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			id: create_query_signal::<String>("id").0.get().unwrap_or_default(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct VideoResource {
	pub resource: Resource<VideoResourceArgs, Result<Video, RustyTubeError>>,
}

impl VideoResource {
	pub fn initialise() -> Self {
		VideoResource {
			resource: Resource::local(
				move || VideoResourceArgs::new(),
				move |args| fetch_video(args),
			),
		}
	}
}

async fn fetch_video(args: VideoResourceArgs) -> Result<Video, RustyTubeError> {
	Video::fetch_video(&args.server, &args.id, &args.locale.to_invidious_lang()).await
}
