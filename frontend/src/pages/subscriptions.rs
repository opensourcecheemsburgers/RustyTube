use invidious::{CommonVideo, SubscriptionsVideos};
use leptos::*;
use rustytube_error::RustyTubeError;

use crate::{
	components::{FerrisError, PlaceholderCardArray, VideoPreviewCard},
	contexts::{SubsVideosCtx, SubscriptionsCtx},
	icons::FerrisWaveIcon,
	pages::settings::ImportSubsButton,
};

#[component]
pub fn SubscriptionsSection() -> impl IntoView {
	let subs = expect_context::<SubscriptionsCtx>().0;

	let subs_view = move || match subs.get().channels.len() == 0 {
		true => view! { <ImportSubscriptions/> },
		false => view! { <SubscriptionsVideos/> },
	};

	view! {
		<div class="flex justify-center w-full mt-4">
			<div class="w-[90%] flex flex-col gap-y-8">
				<h1 class="text-2xl font-semibold">{"Subscriptions"}</h1>
				{subs_view}
			</div>
		</div>
	}
}

#[component]
pub fn SubscriptionsVideos() -> impl IntoView {
	let subs_videos_resource = expect_context::<SubsVideosCtx>().0;

	view! {
		<Suspense fallback=move || {
			view! { <PlaceholderCardArray/> }
		}>
			{move || {
				subs_videos_resource
					.get()
					.map(|subs_videos_res| {
						match subs_videos_res {
							Ok(subs_videos) => {
								view! { <SubscriptionsVideosInner subs_videos=subs_videos/> }
							}
							Err(err) => view! { <FerrisError error=err/> },
						}
					})
			}}

		</Suspense>
	}
}

#[component]
pub fn SubscriptionsVideosInner(subs_videos: SubscriptionsVideos) -> impl IntoView {
	let mut videos: Vec<Vec<CommonVideo>> = Vec::new();
	let mut fails: Vec<RustyTubeError> = Vec::new();

	subs_videos.into_iter().for_each(|sub| match sub {
		Ok(sub_videos) => videos.push(sub_videos.videos),
		Err(error) => fails.push(error),
	});

	let mut total_videos: Vec<CommonVideo> = videos.into_iter().flatten().collect();
	total_videos.sort_by(|a, b| b.published.cmp(&a.published));

	let total_videos_len = total_videos.len();

	let initial_len = match total_videos_len > 100 {
		true => 100,
		false => total_videos_len,
	};
	let initial_videos = Vec::from(&total_videos[0..initial_len]);
	let visible_videos = create_rw_signal(initial_videos);

	let videos_view = move || {
		visible_videos
			.get()
			.into_iter()
			.map(|video| view! { <VideoPreviewCard video=video/> })
			.collect_view()
	};

	let load_more = move |_| load_more_videos(visible_videos, total_videos.clone());

	let view_more_btn = match visible_videos.get().len() == total_videos_len {
		true => view! { <div></div> }.into_view(),
		false => view! {
			<div class="flex justify-center">
				<button on:click=load_more class="btn btn-lg btn-primary btn-outline">
					{"Load More"}
				</button>
			</div>
		}
		.into_view(),
	};

	view! {
		<div class="-ml-4 flex flex-col h-[calc(100vh-11.75rem)] gap-y-8 overflow-y-hidden hover:overflow-y-auto scroll-smooth">
			<div class="flex flex-row flex-wrap justify-between gap-y-8">{videos_view}</div>
			{view_more_btn}
		</div>
	}
}

#[component]
pub fn ImportSubscriptions() -> impl IntoView {
	view! {
		<div class="min-h-full hero">
			<div class="flex flex-col space-y-8">
				<FerrisWaveIcon width=96/>
				<div class="flex flex-row justify-center space-x-4">
					<ImportSubscriptionsTutorial/>
					<ImportSubsButton/>
				</div>
			</div>
		</div>
	}
}

#[component]
pub fn ImportSubscriptionsTutorial() -> impl IntoView {
	view! {
		<a
			target="_blank"
			class="btn btn-lg btn-outline btn-info"
			href="https://docs.invidious.io/export-youtube-subscriptions/"
		>
			{"Tutorial"}
		</a>
	}
}

fn load_more_videos(visible_videos: RwSignal<Vec<CommonVideo>>, total_videos: Vec<CommonVideo>) {
	visible_videos.update(|visible| {
		let next_slice = &total_videos[(visible.len())..(visible.len() + 100)];
		visible.extend_from_slice(next_slice);
	});
}
