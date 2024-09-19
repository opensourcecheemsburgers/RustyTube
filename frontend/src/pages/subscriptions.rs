use gloo::file::Blob;
use invidious::{ChannelVideos, CommonVideo, Subscriptions};
use leptos::{
	component, create_action, create_rw_signal, expect_context, view,
	wasm_bindgen, web_sys, CollectView, IntoView, Props, RwSignal, Show,
	SignalGet, SignalUpdate, Suspense,
};
use rustytube_error::RustyTubeError;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

use crate::{
	components::{
		CardGrid, FerrisError, GridContainer, PlaceholderCardArray,
		VideoPreviewCard,
	},
	icons::FerrisWaveIcon,
	resources::{
		save_subs, SubscriptionsCtx, SubscriptionsThumbnailsResource,
		SubscriptionsVideosResource,
	},
	utils::i18n,
};

#[component]
pub fn SubscriptionsSection() -> impl IntoView {
	let subs = expect_context::<SubscriptionsCtx>();

	view! {
		<GridContainer>
			<h1 class="text-2xl font-semibold">
				{i18n("sidebar.subscriptions")}
			</h1>
			<Suspense fallback=PlaceholderCardArray>
				<Show
					when=move || !subs.0.get().channels.is_empty()
					fallback=ImportSubscriptions
				>
					<SubscriptionsVideos/>
				</Show>
			</Suspense>

		</GridContainer>
	}
}

#[component]
pub fn SubscriptionsVideos() -> impl IntoView {
	let subs_videos_resource =
		expect_context::<SubscriptionsVideosResource>().resource;

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
								view! {
									<SubscriptionsVideosInner subs_videos=subs_videos/>
								}
							}
							Err(err) => view! { <FerrisError error=err/> },
						}
					})
			}}

		</Suspense>
	}
}

#[component]
pub fn SubscriptionsVideosInner(
	subs_videos: Vec<Result<ChannelVideos, RustyTubeError>>,
) -> impl IntoView {
	let mut videos: Vec<Vec<CommonVideo>> = Vec::new();
	let mut fails: Vec<RustyTubeError> = Vec::new();

	subs_videos.into_iter().for_each(|sub| match sub {
		Ok(sub_videos) => videos.push(sub_videos.videos),
		Err(error) => fails.push(error),
	});

	let mut total_videos: Vec<CommonVideo> =
		videos.into_iter().flatten().collect();
	total_videos.sort_by(|a, b| b.published.cmp(&a.published));

	let total_videos_len = total_videos.len();

	let initial_len = if total_videos_len > 100 {
		100
	} else {
		total_videos_len
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

	let load_more = move |_| load_more_videos(visible_videos, &total_videos);

	let view_more_btn = if visible_videos.get().len() == total_videos_len {
		view! { <div></div> }.into_view()
	} else {
		view! {
			<div class="flex justify-center">
				<button
					on:click=load_more
					class="btn btn-lg btn-primary btn-outline"
				>
					{"Load More"}
				</button>
			</div>
		}
		.into_view()
	};

	view! {
		<CardGrid>{videos_view}</CardGrid>
		{view_more_btn}
	}
}

#[component]
pub fn ImportSubscriptions() -> impl IntoView {
	view! {
		<div class="min-h-full hero">
			<div class="flex flex-col space-y-8">
				<FerrisWaveIcon width=96/>
				<div class="flex flex-row gap-x-4 justify-center">
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
			class="btn btn-outline btn-info lg:btn-lg"
			href="https://docs.invidious.io/export-youtube-subscriptions/"
		>
			{i18n("subscriptions.tutorial")}
		</a>
	}
}

fn load_more_videos(
	visible_videos: RwSignal<Vec<CommonVideo>>,
	total_videos: &[CommonVideo],
) {
	visible_videos.update(|visible| {
		let next_slice = &total_videos[(visible.len())..(visible.len() + 100)];
		visible.extend_from_slice(next_slice);
	});
}

#[component]
pub fn ImportSubsButton() -> impl IntoView {
	let subs = expect_context::<SubscriptionsCtx>();

	let parse_subs_file = create_action(|input: &(SubscriptionsCtx, Event)| {
		let subs = input.0;
		let event = input.1.clone();

		get_subs_from_file(subs, event)
	});

	let on_file_upload = move |event: Event| {
		parse_subs_file.dispatch((subs, event));
	};

	view! {
		<div>
			<label class="btn btn-md btn-primary lg:btn-lg" for="subs_upload">
				{i18n("settings.import")}
			</label>
			<input
				id="subs_upload"
				type="file"
				accept=".ron,.json,.csv"
				multiple=false
				on:change=on_file_upload
				class="hidden"
			/>
		</div>
	}
}

async fn get_subs_from_file(
	subs_resource: SubscriptionsCtx,
	event: Event,
) -> Result<(), RustyTubeError> {
	let input = event
		.target()
		.ok_or(RustyTubeError::TargetNotFound)?
		.dyn_into::<HtmlInputElement>()
		.ok()
		.ok_or(RustyTubeError::DynInto)?;
	let filelist = input.files().ok_or(RustyTubeError::NoFileSelected)?;
	let file = filelist.get(0).ok_or(RustyTubeError::NoFileSelected)?;
	let blob: Blob = file.into();
	let mut subscriptions = Subscriptions::read_subs(blob).await?;
	subs_resource.0.update(|subs| {
		subs.channels.append(&mut subscriptions.channels);
		subs.channels
			.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
		subs.channels.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));
		save_subs(subs);
	});
	expect_context::<SubscriptionsVideosResource>().resource.refetch();
	expect_context::<SubscriptionsThumbnailsResource>().resource.refetch();
	Ok(())
}
