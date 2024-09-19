use invidious::{SearchArgs, SearchResult};
use leptos::{
	component, view, CollectView, IntoView, RwSignal, Signal, SignalGet,
	Suspense,
};

use crate::{
	components::{
		CardGrid, ChannelPreviewCard, FerrisError, GridContainer,
		PlaceholderCardArray, PlaylistPreviewCard, VideoPreviewCard,
	},
	resources::{
		get_search_args_from_query_map, SearchAction, SearchActionArgs,
		SearchResource,
	},
	utils::i18n,
};

#[component]
pub fn SearchSection() -> impl IntoView {
	let search_args = get_search_args_from_query_map();
	let search = SearchResource::initialise(search_args);

	view! {
		<GridContainer>
			<h1 class="pl-4 text-2xl font-semibold">
				{move || search_args.get().query}
			</h1>
			<Suspense fallback=move || {
				view! { <PlaceholderCardArray/> }
			}>

				{move || {
					search
						.resource
						.get()
						.map(|response| {
							match response {
								Ok(search_results) => {
									view! {
										<SearchResults
											search_results=search_results
											search_args=search_args
										/>
									}
								}
								Err(err) => view! { <FerrisError error=err/> },
							}
						})
				}}

			</Suspense>
		</GridContainer>
	}
}

#[component]
pub fn SearchResults(
	search_results: Vec<SearchResult>,
	search_args: Signal<SearchArgs>,
) -> impl IntoView {
	let pages = RwSignal::new(vec![search_results]);
	let search_action = SearchAction::new();

	view! {
		<CardGrid>

			{move || {
				pages
					.get()
					.into_iter()
					.map(|result_page| {
						result_page
							.into_iter()
							.map(|result| match result {
								SearchResult::Channel(channel) => {
									view! { <ChannelPreviewCard channel=channel/> }
								}
								SearchResult::Video(video) => {
									view! { <VideoPreviewCard video=video/> }
								}
								SearchResult::Playlist(playlist) => {
									view! { <PlaylistPreviewCard playlist=playlist/> }
								}
							})
							.collect_view()
					})
					.collect_view()
			}}

		</CardGrid>
		<button
			class="btn btn-primary btn-outline btn-sm"
			on:click=move |_| {
				search_action
					.action
					.dispatch(SearchActionArgs::new(search_args, pages));
			}
		>

			{i18n("general.load_more")}
		</button>
	}
}
