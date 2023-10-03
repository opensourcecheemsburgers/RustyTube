use leptos::*;
use invidious::{Subscriptions, CommonVideo, SubscriptionsVideos};
use rustytube_error::RustyTubeError;
use web_sys::{HtmlInputElement, Event};
use gloo::file::Blob;
use wasm_bindgen::JsCast;

use crate::components::{VideoPreviewCard, FerrisError, PlaceholderCardArray};
use crate::contexts::{ServerCtx, SubscriptionsCtx, SubsVideosCtx};
use crate::icons::FerrisWaveIcon;

#[component]
pub fn SubscriptionsSection() -> impl IntoView {
	let subs = expect_context::<SubscriptionsCtx>().0;
	let subs_videos_resource = expect_context::<SubsVideosCtx>().0;

	view! {
		<div class="w-full flex justify-center mt-4">
			<div class="w-[90%] flex flex-col gap-y-8">
				<h1 class="pl-4 font-semibold text-2xl">{"Subscriptions"}</h1>
				<Suspense fallback=move || {
					view! { <PlaceholderCardArray/> }
				}>

					{move || match subs.get().channels.len() == 0 {
						false => {
							subs_videos_resource
								.read()
								.map(|subs_videos_res| {
									match subs_videos_res {
										Ok(subs_videos) => {
											view! { <SubscriptionsVideos subs_videos=subs_videos/> }
										}
										Err(err) => view! { <FerrisError error=err/> },
									}
								})
						}
						true => Some(view! { <ImportSubscriptions/> }),
					}}

				</Suspense>
			</div>
		</div>
	}
}

#[component]
pub fn SubscriptionsVideos(subs_videos: SubscriptionsVideos) -> impl IntoView {
	let mut videos: Vec<Vec<CommonVideo>> = Vec::new();
	let mut fails: Vec<RustyTubeError> = Vec::new();

	subs_videos.into_iter().for_each(|sub| {
		match sub {
			Ok(sub_videos) => videos.push(sub_videos),
			Err(error) => fails.push(error)
		}
	});

	let mut total_videos: Vec<CommonVideo> = videos.into_iter().flatten().collect();
	total_videos.sort_by(|a, b| b.published.cmp(&a.published));

	let total_videos_len = total_videos.len();

	let initial_len = match total_videos_len > 100 {
		true => 100,
		false => total_videos_len
	};
	let initial_videos = Vec::from(&total_videos[0..initial_len]);
	let visible_videos = create_rw_signal(initial_videos);

	let videos_view = move || {
		visible_videos.get().into_iter().map(|video| view! { <VideoPreviewCard video=video/> }
		).collect_view()
	};

	let load_more = move |_| { load_more_videos(visible_videos, total_videos.clone()) };

	let view_more_btn = match visible_videos.get().len() == total_videos_len {
		true => view! { <div></div> }.into_view(),
		false => view! {
			<div class="flex justify-center">
				<button on:click=load_more class="btn btn-lg btn-primary btn-outline">
					{"Load More"}
				</button>
			</div>
		}.into_view()
	};

	view! {
		<div class="flex flex-col h-[calc(100vh-64px-1rem-128px)] gap-y-8 overflow-y-auto scroll-smooth">
			<div class="flex flex-row flex-wrap gap-y-8 justify-between">{videos_view}</div>
			{view_more_btn}
		</div>
	}
}

#[component]
pub fn ImportSubscriptions() -> impl IntoView {
	view! {
		<div class="hero min-h-full">
			<div class="flex flex-col space-y-8">
				<FerrisWaveIcon width=96/>
				<div class="flex flex-row space-x-4">
					<ImportSubscriptionsTutorial/>
					<ImportSubscriptionsBtn/>
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

#[component]
pub fn ImportSubscriptionsBtn() -> impl IntoView {
	let subs = expect_context::<SubscriptionsCtx>().0.write_only();

	let parse_subs_file = create_action(|input: &(WriteSignal<Subscriptions>, Event)| {
		let subs = input.0.clone();
		let event = input.1.clone();

		get_subs_from_file(subs, event)
	});

	let on_file_upload = move |event: Event| {
		parse_subs_file.dispatch((subs, event));
	};

	view! {
		<>
			<label class="btn btn-lg btn-outline btn-primary" for="subs_upload">
				{"Import Subscriptions"}
			</label>
			<input
				id="subs_upload"
				type="file"
				accept=".ron,.json,.csv"
				multiple=false
				on:change=on_file_upload
				class="hidden"
			/>
		</>
	}
}

async fn get_subs_from_file(subs: WriteSignal<Subscriptions>, event: Event) -> Result<(), RustyTubeError> {
	let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
	let filelist = input.files().ok_or(RustyTubeError::no_file_selected())?;
	let file = filelist.get(0).ok_or(RustyTubeError::no_file_selected())?;
	let blob: Blob = file.into();
	let subscriptions = Subscriptions::read_subs(blob).await?;
	subscriptions.save().await?;
	subs.set(subscriptions);
	Ok(())
}

fn load_more_videos(visible_videos: RwSignal<Vec<CommonVideo>>, total_videos: Vec<CommonVideo>) {
	visible_videos
		.update(|visible| {
			let next_slice = &total_videos[(visible.len())..(visible.len() + 100)];
			visible.extend_from_slice(next_slice);
		});
}









