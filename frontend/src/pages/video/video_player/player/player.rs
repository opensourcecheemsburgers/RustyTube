use std::time::Duration;

use invidious::{Format, Formats, Video, VideoFormat};
use leptos::{leptos_dom::helpers::TimeoutHandle, *};
use phosphor_leptos::{IconWeight, SpinnerGap};
use utils::get_element_by_id;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlDivElement};

use crate::{
	components::FerrisError,
	contexts::{
		PlaybackState, PlayerState, PlayerStyle, VIDEO_CONTAINER_ID,
		VIDEO_CONTROLS_ID,
	},
	pages::video::{
		utils::get_format,
		video_player::{
			player::{audio::AudioStream, video::VideoStream},
			VideoPlayerControls,
		},
	},
	resources::VideoResource,
};

#[component]
pub fn VideoContainer() -> impl IntoView {
	provide_context(PlayerState::init());
	provide_context(PlayerStyle::init());

	view! {
		<Suspense fallback=move || {
			view! { <VideoPlaceholder/> }
		}>
			{move || {
				expect_context::<VideoResource>()
					.resource
					.get()
					.map(|video_result| match video_result {
						Ok(video) => view! { <VideoPlayer video=video/> },
						Err(err) => view! { <FerrisError error=err/> },
					})
			}}

		</Suspense>
	}
}

#[component]
pub fn VideoPlayer(video: Video) -> impl IntoView {
	let state = expect_context::<PlayerState>();
	let style = expect_context::<PlayerStyle>();

	let formats = Formats::from((
		video.adaptive_formats.clone(),
		video.format_streams.clone(),
	));
	let format = get_format(&formats).ok();
	provide_context(create_rw_signal(formats));
	provide_context::<RwSignal<Option<Format>>>(create_rw_signal(format));

	// let format = get_video_format_ctx(&formats).ok();
	// provide_context(create_rw_signal(format));

	let handle_store: RwSignal<Option<TimeoutHandle>> = create_rw_signal(None);

	let idle_detection = move |_| {
		style.controls_visible.set(true);
		if let Some(handle) = handle_store.get() {
			handle.clear();
		}
		let handle = set_timeout_with_handle(
			move || {
				style.controls_visible.set(cursor_visible());
			},
			Duration::from_secs(3),
		);
		if let Ok(handle) = handle {
			handle_store.set(Some(handle));
		}
	};

	set_interval(
		move || {
			let _ = state.sync();
		},
		Duration::from_secs(3),
	);

	view! {
		<div
			dir="ltr"
			data-controls=move || style.controls_visible.get().to_string()
			data-fullwindow=move || style.full_window.get().to_string()

			on:click=move |_| {
				if !controls_hovered() {
					let _ = state.toggle_playback();
				}
			}

			on:dblclick=move |_| {
				if !controls_hovered() {
					toggle_fullscreen();
				}
			}

			on:mouseover=idle_detection
			on:mousemove=idle_detection
			class=VIDEO_CLASSES
			id=VIDEO_CONTAINER_ID
		>
			<VideoStream video=video/>
			<AudioStream/>
			<VideoPlayerControls/>
			<LoadingCircle/>
		</div>
	}
}

#[component]
pub fn VideoPlaceholder() -> impl IntoView {
	view! {
		<div class="flex flex-col justify-center items-center w-full rounded bg-base-300">
			<div class="w-full rounded animate-pulse aspect-video bg-base-300"></div>
		</div>
	}
}

#[component]
pub fn VideoFormat() -> impl IntoView {
	let url = expect_context::<RwSignal<Option<VideoFormat>>>()
		.get()
		.map(|format| format.url);

	move || view! { <source src=url.clone()/> }
}

#[component]
pub fn LoadingCircle() -> impl IntoView {
	let state = expect_context::<PlayerState>();
	let classes = move || {
		if state.playback_state.get() == PlaybackState::Loading {
			"flex flex-col items-center absolute -translate-x-1/2 -translate-y-1/2 top-2/4 left-1/2"
		} else {
			"hidden"
		}
	};

	view! {
		<div role="status" class=classes>
			<SpinnerGap weight=IconWeight::Regular class="w-16 h-16"/>
		</div>
	}
}

fn cursor_visible() -> bool {
	!document().fullscreen() || controls_hovered()
}

fn controls_hovered() -> bool {
	let hovered_elements = document().query_selector_all(":hover").ok();

	hovered_elements.map_or(false, |hovered_elements| {
		let mut elements_vec = vec![];
		let mut index = 0;
		while let Some(node) = hovered_elements.item(index) {
			if let Ok(element) = node.dyn_into::<Element>() {
				elements_vec.push(element);
				index += 1;
			}
		}
		elements_vec.iter().any(|element| element.id().eq(VIDEO_CONTROLS_ID))
	})
}

fn toggle_fullscreen() {
	if document().fullscreen() {
		document().exit_fullscreen();
	} else if let Ok(element) =
		get_element_by_id::<HtmlDivElement>(VIDEO_CONTAINER_ID)
	{
		element.request_fullscreen();
	}
}

pub const VIDEO_CLASSES: &str = "\
relative flex flex-col transition-all \
object-contain items-center justify-center \
\
data-[controls=false]:cursor-none \
data-[fullwindow=false]:w-full \
data-[fullwindow=false]:duration-300 \
data-[fullwindow=false]:ease-out \
\
data-[fullwindow=true]:fullwindow \
data-[fullwindow=true]:object-cover \
data-[fullwindow=true]:ease-in \
data-[fullwindow=true]:duration-300 \
";
