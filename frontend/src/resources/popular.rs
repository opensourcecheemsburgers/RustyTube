use invidious::Popular;
use leptos::{expect_context, Resource, SignalGet};
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

use super::save_resource;

static POPULAR_KEY: &str = "popular_videos";

#[derive(Clone, PartialEq, Eq)]
pub struct PopularResourceArgs {
	server: String,
	locale: RustyTubeLocale,
}

impl PopularResourceArgs {
	fn new() -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct PopularResource {
	pub resource:
		Resource<PopularResourceArgs, Result<Popular, RustyTubeError>>,
}

impl PopularResource {
	pub fn initialise() -> Self {
		let resource = Resource::local(PopularResourceArgs::new, fetch_popular);

		Self { resource }
	}
}

async fn fetch_popular(
	args: PopularResourceArgs,
) -> Result<Popular, RustyTubeError> {
	let popular =
		Popular::fetch_popular(&args.server, args.locale.to_invidious_lang())
			.await?;
	save_resource(POPULAR_KEY, &popular).await?;
	Ok(popular)
}
