use std::str::FromStr;

use invidious::{
	Duration, Feature, ResponseType, SearchArgs, SearchResult, SearchResults,
	Sort, Suggestions, TimeSpan,
};
use leptos::{
	expect_context, Action, Resource, RwSignal, Signal, SignalGet, SignalUpdate,
};
use leptos_router::use_query_map;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

#[derive(Clone, PartialEq, Eq)]
pub struct SearchResourceArgs {
	server: String,
	locale: RustyTubeLocale,
	search_args: SearchArgs,
}

impl SearchResourceArgs {
	pub fn get(search_args: Signal<SearchArgs>) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			search_args: search_args.get(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct SearchResource {
	pub resource:
		Resource<SearchResourceArgs, Result<Vec<SearchResult>, RustyTubeError>>,
}

impl SearchResource {
	pub fn initialise(search_args: Signal<SearchArgs>) -> Self {
		Self {
			resource: Resource::local(
				move || SearchResourceArgs::get(search_args),
				fetch_search_results,
			),
		}
	}
}

async fn fetch_search_results(
	args: SearchResourceArgs,
) -> Result<Vec<SearchResult>, RustyTubeError> {
	Ok(SearchResults::fetch_search_results(
		&args.server,
		&args.search_args,
		1,
		args.locale.to_invidious_lang(),
	)
	.await?
	.items)
}

#[derive(Clone, PartialEq, Eq)]
pub struct SearchActionArgs {
	server: String,
	locale: RustyTubeLocale,
	search_args: SearchArgs,
	pages: RwSignal<Vec<Vec<SearchResult>>>,
	page_number: u32,
}

impl SearchActionArgs {
	#[allow(clippy::cast_possible_truncation)]
	pub fn new(
		search_args: Signal<SearchArgs>,
		pages: RwSignal<Vec<Vec<SearchResult>>>,
	) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			search_args: search_args.get(),
			pages,
			page_number: (pages.get().len() + 1) as u32,
		}
	}
}

#[derive(Clone, Copy)]
pub struct SearchAction {
	pub action: Action<SearchActionArgs, Result<(), RustyTubeError>>,
}

impl SearchAction {
	pub fn new() -> Self {
		Self {
			action: Action::new(|args: &SearchActionArgs| {
				fetch_more_search_results(args.clone())
			}),
		}
	}
}

#[allow(clippy::cast_possible_truncation)]
async fn fetch_more_search_results(
	args: SearchActionArgs,
) -> Result<(), RustyTubeError> {
	let page_number = move || (args.pages.get().len() + 1) as u32;
	let search_results = SearchResults::fetch_search_results(
		&args.server,
		&args.search_args,
		page_number(),
		args.locale.to_invidious_lang(),
	)
	.await?;
	args.pages.update(|pages| pages.push(search_results.items));
	Ok(())
}

#[derive(Clone, PartialEq, Eq)]
pub struct SearchSuggestionsResourceArgs {
	server: String,
	locale: RustyTubeLocale,
	query: String,
}

impl SearchSuggestionsResourceArgs {
	pub fn new(query: RwSignal<String>) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			query: query.get(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct SearchSuggestions {
	pub resource: Resource<
		SearchSuggestionsResourceArgs,
		Result<Suggestions, RustyTubeError>,
	>,
}

impl SearchSuggestions {
	pub fn initialise(query: RwSignal<String>) -> Self {
		Self {
			resource: Resource::local(
				move || SearchSuggestionsResourceArgs::new(query),
				fetch_search_suggestions,
			),
		}
	}
}

pub async fn fetch_search_suggestions(
	args: SearchSuggestionsResourceArgs,
) -> Result<Suggestions, RustyTubeError> {
	Suggestions::fetch_suggestions(
		&args.query,
		&args.server,
		args.locale.to_invidious_lang(),
	)
	.await
}

pub fn get_search_args_from_query_map() -> Signal<SearchArgs> {
	Signal::derive(move || {
		let map = use_query_map().get();
		let query = map.get("q").cloned().unwrap_or_default();
		let response_type = map.get("type").and_then(|response_type| {
			ResponseType::from_str(response_type).ok()
		});
		let sort = map.get("sort").and_then(|sort| Sort::from_str(sort).ok());
		let timespan = map
			.get("timespan")
			.and_then(|timespan| TimeSpan::from_str(timespan).ok());
		let duration = map
			.get("duration")
			.and_then(|duration| Duration::from_str(duration).ok());
		let features = map.get("features").map(|features| {
			let mut features_vec = vec![];
			features.split(',').for_each(|feature| {
				if let Ok(feature) = Feature::from_str(feature) {
					features_vec.push(feature);
				}
			});
			features_vec
		});
		SearchArgs::new(
			query,
			sort,
			timespan,
			duration,
			response_type,
			features,
		)
	})
}
