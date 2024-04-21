use invidious::{Trending, TrendingCategory};
use isocountry::CountryCode;
use leptos::*;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

use super::{initial_value, save_resource};

static TRENDING_KEY: &'static str = "trending_videos";

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TrendingResourceArgs {
	server: Signal<String>,
	category: RwSignal<TrendingCategory>,
	locale: Signal<RustyTubeLocale>,
	region: Signal<CountryCode>,
}

impl TrendingResourceArgs {
	fn new(category: RwSignal<TrendingCategory>) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0,
			category,
			locale: expect_context::<RegionConfigCtx>().locale_slice.0,
			region: expect_context::<RegionConfigCtx>().trending_region_slice.0,
		}
	}
}

pub struct TrendingResource {
	pub resource: Resource<TrendingResourceArgs, Result<Trending, RustyTubeError>>,
}

impl TrendingResource {
	pub fn initialise(category: RwSignal<TrendingCategory>) -> Self {
		let resource = create_resource_with_initial_value(
			move || TrendingResourceArgs::new(category),
			move |args| fetch_trending(args),
			initial_value(TRENDING_KEY),
		);

		TrendingResource { resource }
	}
}

async fn fetch_trending(args: TrendingResourceArgs) -> Result<Trending, RustyTubeError> {
	let trending = Trending::fetch_trending(
		&args.server.get(),
		&args.category.get(),
		&args.region.get().alpha2(),
		&args.locale.get().to_invidious_lang(),
	)
	.await?;
	save_resource(TRENDING_KEY, &trending).await?;
	Ok(trending)
}
