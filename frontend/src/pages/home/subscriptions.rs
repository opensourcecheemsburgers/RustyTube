use leptos::*;
use invidious::{Subscriptions, CommonVideo, SubscriptionsVideos};
use rustytube_error::RustyTubeError;
use web_sys::{HtmlInputElement, Event};
use gloo::file::Blob;
use js_sys::Reflect::get;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::components::{VideoPreviewCard, VideoPreviewCardPlaceholderArray, FerrisError};
use crate::contexts::{ServerCtx, SubscriptionsCtx};
use crate::pages::home::homepage::{HomepageSection, HomepageSectionTitle};
use crate::icons::FerrisWaveIcon;

#[component]
pub fn SubscriptionsSection(cx: Scope) -> impl IntoView {
	let subs = expect_context::<SubscriptionsCtx>(cx).0;

	let subscriptions_content = move || match subs.get().channels.len() == 0 {
		false => { view! {cx, <SubscriptionsContent subs=subs.read_only() />} },
		true => { view! {cx, <ImportSubscriptions subs=subs.write_only() />} }
	};

	view! {cx,
		<HomepageSection>
			<HomepageSectionTitle title={"Subscriptions".to_string()}/>
				{ subscriptions_content }
		</HomepageSection>
	}
}

#[component]
pub fn SubscriptionsContent(cx: Scope, subs: ReadSignal<Subscriptions>) -> impl IntoView {
	let server = expect_context::<ServerCtx>(cx).0.0;

	let subs = create_resource(
		cx,
		move || (server.get(), subs.get()),
		|(server, subs)| async move {
			subs.fetch_subs(&server).await
		},
	);

	let subscriptions_view = move || match subs.read(cx) {
		Some(subs) => {
			match subs {
				Ok(subs) => {view! {cx, <SubscriptionsVideos subs=subs/>}},
				Err(err) => {view! {cx, <SubscriptionsError error=err/>}}
			}
		},
		None => view! {cx, <VideoPreviewCardPlaceholderArray />}.into_view(cx)
	};


	view! {cx, {subscriptions_view} }
}


#[component]
pub fn SubscriptionsVideos(cx: Scope, subs: SubscriptionsVideos) -> impl IntoView {
	let mut videos: Vec<Vec<CommonVideo>> = Vec::new();
	let mut fails: Vec<RustyTubeError> = Vec::new();

	subs.into_iter().for_each(|sub| {
		match sub {
			Ok(sub_videos) => videos.push(sub_videos),
			Err(error) => fails.push(error)
		}
	});

	let videos: Vec<CommonVideo> = videos.into_iter().flatten().collect();

	let videos_view = move || {
		videos.clone().into_iter().map(|video| view!
			{ cx,
                <VideoPreviewCard
                    title=video.title
                    author=video.author
                    views=video.views
                    published=video.published_text
                    thumbnail_url=video.thumbnails.last().unwrap().url.clone()
                />
			}
		).collect_view(cx)
	};

	view! {cx,
        <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-1rem-128px)] pb-12 overflow-y-auto scroll-smooth">
			{ videos_view }
        </div>
    }
}
#[component]
pub fn ImportSubscriptions(cx: Scope, subs: WriteSignal<Subscriptions>) -> impl IntoView {
	view! {cx,
		<div class="hero min-h-full">
			<div class="flex flex-col space-y-8">
				<FerrisWaveIcon width=96/>
				<div class="flex flex-row space-x-4">
					<ImportSubscriptionsTutorial />
					<ImportSubscriptionsBtn subs=subs />
				</div>
			</div>
		</div>
	}
}

#[component]
pub fn ImportSubscriptionsTutorial(cx: Scope) -> impl IntoView {
	view! {cx,
		<a target="_blank" class="btn btn-lg btn-outline btn-info" href="https://docs.invidious.io/export-youtube-subscriptions/">
			{"Tutorial"}
		</a>
	}
}
#[component]
pub fn ImportSubscriptionsBtn(cx: Scope, subs: WriteSignal<Subscriptions>) -> impl IntoView {
	let parse_subs_file = create_action(cx, |input: &(WriteSignal<Subscriptions>, Event)| {
		let subs = input.0.clone();
		let event = input.1.clone();

		get_subs_from_file(subs, event)
	});

	let on_file_upload = move |event: Event| {
		parse_subs_file.dispatch((subs, event));
	};

	view! {cx,
		<>
            <label class="btn btn-lg btn-outline btn-primary" for="subs_upload">
                {"Import Subscriptions"}
            </label>
            <input
				id="subs_upload"
				type="file"
				accept="text/*"
				multiple={false}
				on:change=on_file_upload
				class="hidden" />
		</>
	}
}

#[component]
pub fn SubscriptionsError(cx: Scope, error: RustyTubeError) -> impl IntoView {
	view! {cx,
        <div class="h-[calc(100vh-64px-1rem-128px)]">
            <FerrisError error=error width=96 />
        </div>
    }
}
async fn get_subs_from_file(subs: WriteSignal<Subscriptions>, event: Event) -> Result<(), RustyTubeError> {
	let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
	let filelist = input.files().ok_or(RustyTubeError::no_file_selected())?;
	let file = filelist.get(0).ok_or(RustyTubeError::no_file_selected())?;
	let blob: Blob = file.into();
	let bytes = gloo::file::futures::read_as_text(&blob).await?;
	let subscriptions = Subscriptions::from_ron_str(&bytes).await?;
	subscriptions.save().await?;
	subs.set(subscriptions);
	Ok(())
}