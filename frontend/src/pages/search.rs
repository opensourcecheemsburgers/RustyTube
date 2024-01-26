use std::str::FromStr;

use invidious::{
	Duration, Feature, ResponseType, SearchArgs, SearchResult, SearchResults, Sort, TimeSpan,
};
use leptos::*;
use leptos_router::{use_query_map, ParamsMap};
use rustytube_error::RustyTubeError;

use crate::{
	components::{
		ChannelPreviewCard, FerrisError, PlaceholderCardArray, PlaylistPreviewCard,
		VideoPreviewCard,
	},
	contexts::ServerCtx,
	utils::{get_current_video_query_signal, VideoQuerySignal},
};

#[component]
pub fn SearchSection() -> impl IntoView {
	let server = expect_context::<ServerCtx>().0 .0;

	let query_map = use_query_map();
	let search_args = move || get_search_args_from_query_map(query_map.get());

	let search_results_resource = create_resource(
		move || (server.get(), search_args()),
		|(server, search_args)| async move {
			SearchResults::fetch_search_results(&server, search_args, 1).await
		},
	);

	let title = move || search_args().query;

	view! {
		<div class="w-full flex justify-center mt-4">
			<div class="w-[90%] flex flex-col gap-y-8">
				<h1 class="pl-4 font-semibold text-2xl">{title}</h1>
				// <TrendingHeader category=category/>
				<Suspense fallback=move || {
					view! { <PlaceholderCardArray/> }
				}>
					{move || {
						search_results_resource
							.get()
							.map(|search_results_response| {
								match search_results_response {
									Ok(search_results) => {
										view! {
											<SearchResults
												search_results=search_results
												search_args=search_args()
											/>
										}
									}
									Err(err) => view! { <FerrisError error=err/> },
								}
							})
					}}

				</Suspense>
			</div>
		</div>
	}
}

#[component]
pub fn SearchResults(search_results: SearchResults, search_args: SearchArgs) -> impl IntoView {
	let results_vec = create_rw_signal(vec![search_results.items]);
	let fetch_search_results = create_action(|input: &SearchResultFetchArgs| {
		let args = input.clone();
		async move { fetch_search_results(args).await }
	});
	let server = expect_context::<ServerCtx>().0 .0;
	let video_id = get_current_video_query_signal();

	let search_results_fetch_args =
		SearchResultFetchArgs { search_args, results_vec, video_id, server, fetch_search_results };

	let fetch_results = move |_| fetch_search_results.dispatch(search_results_fetch_args.clone());

	let search_results_view = move || {
		results_vec
			.get()
			.into_iter()
			.map(|result_page| {
				result_page
					.into_iter()
					.map(|result| match result {
						SearchResult::Channel(channel) => {
							view! { <ChannelPreviewCard channel=channel/> }
						}
						SearchResult::Video(video) => view! { <VideoPreviewCard video=video/> },
						SearchResult::Playlist(playlist) => {
							view! { <PlaylistPreviewCard playlist=playlist/> }
						}
					})
					.collect_view()
			})
			.collect_view()
	};

	view! {
		<div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-4rem-128px)] pb-12 overflow-y-hidden hover:overflow-y-auto scroll-smooth">
			{search_results_view}
			<button class="btn btn-primary btn-outline btn-sm" on:click=fetch_results>
				{"Load more"}
			</button>
		</div>
	}
}

fn get_search_args_from_query_map(map: ParamsMap) -> SearchArgs {
	let query = map.get("q").cloned().unwrap_or_default();
	let response_type =
		map.get("type").map(|response_type| ResponseType::from_str(response_type).ok()).flatten();
	let sort = map.get("sort").map(|sort| Sort::from_str(sort).ok()).flatten();
	let timespan = map.get("timespan").map(|timespan| TimeSpan::from_str(timespan).ok()).flatten();
	let duration = map.get("duration").map(|duration| Duration::from_str(duration).ok()).flatten();
	let features = map.get("features").map(|features| {
		let mut features_vec = vec![];
		features.split(',').for_each(|feature| {
			if let Ok(feature) = Feature::from_str(feature) {
				features_vec.push(feature)
			}
		});
		features_vec
	});
	SearchArgs::new(query, sort, timespan, duration, response_type, features)
}

async fn fetch_search_results(args: SearchResultFetchArgs) -> Result<(), RustyTubeError> {
	let page_number = (args.results_vec.get().len() + 1) as u32;
	let search_results =
		SearchResults::fetch_search_results(&args.server.get(), args.search_args, page_number)
			.await
			.unwrap();
	let mut temp = args.results_vec.get();
	temp.push(search_results.items);
	args.results_vec.set(temp);
	Ok(())
}

#[derive(Clone)]
pub struct SearchResultFetchArgs {
	pub search_args: SearchArgs,
	pub results_vec: RwSignal<Vec<Vec<SearchResult>>>,
	pub server: Signal<String>,
	pub video_id: VideoQuerySignal,
	pub fetch_search_results: Action<Self, Result<(), RustyTubeError>>,
}
// #[component]
// pub fn VideoResults(videos: Vec<CommonVideo>) -> impl IntoView {
//     let videos_view = videos.into_iter().map(|video| view! { <VideoPreviewCard video=video/>
// }).collect_view();

//     view! {
//         <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-4rem-128px)] pb-12
// overflow-y-auto scroll-smooth">             {videos_view}
//         </div>
//     }
// }

// #[component]
// pub fn ChannelResults(channels: Vec<CommonChannel>) -> impl IntoView {
//     let channels_view = channels.into_iter().map(|channel| view! { <ChannelPreviewCard
// channel=channel/> }).collect_view();

//     view! {
//         <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-4rem-128px)] pb-12
// overflow-y-auto scroll-smooth">             {channels_view}
//         </div>
//     }
// }

// #[component]
// pub fn PlaylistResults(playlists: Vec<CommonPlaylist>) -> impl IntoView {
//     let playlists_view = playlists.into_iter().map(|playlist| view! { <PlaylistPreviewCard
// playlist=playlist/> }).collect_view();

//     view! {
//         <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-4rem-128px)] pb-12
// overflow-y-auto scroll-smooth">             {playlists_view}
//         </div>
//     }
// }
