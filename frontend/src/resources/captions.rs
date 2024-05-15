use invidious::{Captions, Video};
use leptos::*;
use leptos_router::create_query_signal;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

static POPULAR_KEY: &'static str = "popular_videos";

#[derive(Clone, PartialEq)]
pub struct CaptionsResourceArgs {
	server: String,
	id: String,
}

impl CaptionsResourceArgs {
	fn new() -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			id: create_query_signal::<String>("id").0.get().unwrap_or_default(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct CaptionsResource {
	pub resource: Resource<CaptionsResourceArgs, Result<Captions, RustyTubeError>>,
}

impl CaptionsResource {
	pub fn initialise() -> Self {
		CaptionsResource {
			resource: Resource::local(
				move || CaptionsResourceArgs::new(),
				move |args| fetch_captions(args),
			),
		}
	}
}

async fn fetch_captions(args: CaptionsResourceArgs) -> Result<Captions, RustyTubeError> {
	Captions::fetch_captions(&args.server, &args.id).await
}
