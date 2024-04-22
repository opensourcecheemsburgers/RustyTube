use invidious::Popular;
use leptos::*;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

use super::{initial_value, save_resource};

static POPULAR_KEY: &'static str = "popular_videos";

#[derive(Clone, PartialEq)]
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
	let popular = Popular::fetch_popular(&args.server, &args.locale.to_invidious_lang()).await?;
	save_resource(POPULAR_KEY, &popular).await?;
	Ok(popular)
}
