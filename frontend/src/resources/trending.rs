use invidious::{Trending, TrendingCategory};
use isocountry::CountryCode;
use leptos::*;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

use super::save_resource;

static TRENDING_KEY: &'static str = "trending_videos";

#[derive(Clone, PartialEq)]
pub struct TrendingResourceArgs {
	pub server: String,
	pub category: TrendingCategory,
	pub locale: RustyTubeLocale,
	pub region: CountryCode,
}

impl TrendingResourceArgs {
	fn new(category: RwSignal<TrendingCategory>) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			category: category.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			region: expect_context::<RegionConfigCtx>().trending_region_slice.0.get(),
		}
	}
}

pub struct TrendingResource {
	pub resource: Resource<TrendingResourceArgs, Result<Trending, RustyTubeError>>,
}

impl TrendingResource {
	pub fn initialise(category: RwSignal<TrendingCategory>) -> Self {
		let resource = Resource::local(
			move || TrendingResourceArgs::new(category),
			move |args| fetch_trending(args),
		);

		TrendingResource { resource }
	}
}

async fn fetch_trending(args: TrendingResourceArgs) -> Result<Trending, RustyTubeError> {
	let trending = Trending::fetch_trending(
		&args.server,
		&args.category,
		&args.region.alpha2(),
		&args.locale.to_invidious_lang(),
	)
	.await?;
	save_resource(TRENDING_KEY, &trending).await?;
	Ok(trending)
}
