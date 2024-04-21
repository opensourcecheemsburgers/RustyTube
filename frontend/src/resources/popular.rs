use invidious::Popular;
use leptos::*;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

use super::{initial_value, save_resource};

static POPULAR_KEY: &'static str = "popular_videos";

#[derive(Clone, Copy, PartialEq)]
pub struct PopularResourceArgs {
	server: Signal<String>,
	locale: Signal<RustyTubeLocale>,
}

impl PopularResourceArgs {
	fn new() -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0,
			locale: expect_context::<RegionConfigCtx>().locale_slice.0,
		}
	}
}

#[derive(Clone, Copy)]
pub struct PopularResource {
	pub resource: Resource<PopularResourceArgs, Result<Popular, RustyTubeError>>,
}

impl PopularResource {
	pub fn initialise() -> Self {
		let resource = create_resource_with_initial_value(
			move || PopularResourceArgs::new(),
			move |args| fetch_popular(args),
			initial_value(POPULAR_KEY),
		);

		PopularResource { resource }
	}
}

async fn fetch_popular(args: PopularResourceArgs) -> Result<Popular, RustyTubeError> {
	let popular =
		Popular::fetch_popular(&args.server.get(), &args.locale.get().to_invidious_lang()).await?;
	save_resource(POPULAR_KEY, &popular).await?;
	Ok(popular)
}
