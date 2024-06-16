use invidious::Captions;
use leptos::{expect_context, Resource, SignalGet};
use leptos_router::create_query_signal;
use rustytube_error::RustyTubeError;

use crate::contexts::NetworkConfigCtx;

static POPULAR_KEY: &str = "popular_videos";

#[derive(Clone, PartialEq, Eq)]
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
	pub resource:
		Resource<CaptionsResourceArgs, Result<Captions, RustyTubeError>>,
}

impl CaptionsResource {
	pub fn initialise() -> Self {
		Self {
			resource: Resource::local(
				CaptionsResourceArgs::new,
				fetch_captions,
			),
		}
	}
}

async fn fetch_captions(
	args: CaptionsResourceArgs,
) -> Result<Captions, RustyTubeError> {
	Captions::fetch_captions(&args.server, &args.id).await
}
