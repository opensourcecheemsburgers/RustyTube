use std::time::Duration;
use gloo::timers::future::TimeoutFuture;
use leptos::*;
use leptos::leptos_dom::helpers::TimeoutHandle;
use rustytube_error::RustyTubeError;
use utils::get_element_by_id;
use wasm_bindgen::JsCast;
use web_sys::{Event, Element, NodeList, HtmlVideoElement, HtmlDivElement};
use config::Config;
use invidious::{Formats, Video, VideoFormat, Container, LegacyFormat, AdaptiveFormat};

use crate::components::FerrisError;
use crate::contexts::{ServerCtx, VideoFormatCtx, PlayerStyle, PlayerState, VIDEO_PLAYER_ID, AUDIO_PLAYER_ID, PlaybackState, VIDEO_CONTROLS_ID, VIDEO_CONTAINER_ID};
use crate::pages::video::page::VideoResource;
use crate::pages::video::video_player::VideoPlayerControls;

#[component]
pub fn VideoContainer(video_resource: VideoResource) -> impl IntoView {
    let video_player_view = move || video_resource.get().map(|video_result| {
        match video_result {
            Ok(video) => view! { <VideoPlayer video=video/> },
            Err(err) => view! { <FerrisError error=err/> },
        }
    });
    let fallback = move || view! { <VideoPlaceholder/> };

    view! { <Suspense fallback=fallback>{video_player_view}</Suspense> }
}

#[component]
pub fn VideoPlayer(video: Video) -> impl IntoView {
    let server = expect_context::<ServerCtx>().0.0;

    let video_player = create_node_ref::<leptos::html::Video>();
    let audio_player = create_node_ref::<leptos::html::Audio>();

    let state = PlayerState::init(video_player, audio_player);
    let style = PlayerStyle::init();
    provide_context(state);
    provide_context(style);


    let formats = Formats::from((video.adaptive_formats, video.format_streams));
    let webm_dash_formats = filter_webm_dash_formats(&formats.video_formats);
    let format = provide_video_format_ctx(&formats);

    let handle_store: RwSignal<Option<TimeoutHandle>> = create_rw_signal(None);

    let idle_detection = move |_| {
        style.controls_visible.set(true);
        if let Some(handle) = handle_store.get() {
            handle.clear();
        }
        let handle = set_timeout_with_handle(move || {
            style.controls_visible.set(cursor_visible());
        }, Duration::from_secs(3)).unwrap();
        handle_store.set(Some(handle));
    };

	view! {
        <div
            data-controls=style.controls_visible
            data-fullwindow=style.full_window
            on:click=move |_| {
                state.toggle_playback();
            }

            on:dblclick=move |_| toggle_fullscreen()
            on:mouseover=idle_detection
            on:mousemove=idle_detection
            class=VIDEO_CLASSES
            id=VIDEO_CONTAINER_ID
        >
            <video
                _ref=video_player
                on:waiting=move |_| state.set_video_ready(false)
                on:canplay=move |_| state.set_video_ready(true)
                class="w-full rounded"
                id=VIDEO_PLAYER_ID
                on:timeupdate=move |_| {
                    state.update_time();
                }

                poster=&video.thumbnails.first().unwrap().url
                preload="auto"
                controls=false
                autoplay=false
                playsinline
            >
                <VideoFormat/>

                {video
                    .captions
                    .iter()
                    .map(|captions| {
                        view! {
                            <track
                                id=captions.language.clone()
                                src=captions.url(&server.get())
                                srclang=captions.language.clone()
                                label=captions.label.clone()
                            />
                        }
                    })
                    .collect_view()}

            </video>
            <audio
                _ref=audio_player
                on:waiting=move |_| state.set_audio_ready(false)
                on:canplay=move |_| state.set_audio_ready(true)
                id=AUDIO_PLAYER_ID
                src=formats.audio_formats.first().unwrap().url.clone()
                preload="auto"
                controls=false
                autoplay=false
            ></audio>
            <VideoPlayerControls captions=video.captions formats=webm_dash_formats/>
            <LoadingCircle state=state/>
        </div>
    }
}

#[component]
pub fn VideoPlaceholder() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col justify-center items-center bg-base-300 rounded">
            <div class="w-full aspect-w-16 aspect-h-9 bg-base-300 rounded animate-pulse"></div>
        </div>
    }
}


#[component]
pub fn VideoFormat() -> impl IntoView {
    let src = expect_context::<VideoFormatCtx>().0.get().url;
    move || view! { <source src=src.clone()/> }
}


#[component]
pub fn LoadingCircle(state: PlayerState) -> impl IntoView {
    move || {
        match state.playback_state.get() == PlaybackState::Loading {
            true => view! {
                <div
                    role="status"
                    class="absolute -translate-x-1/2 -translate-y-1/2 top-2/4 left-1/2"
                >
                    <svg
                        aria-hidden="true"
                        class="animate-spin w-8 h-8 mr-2 fill-primary"
                        viewBox="0 0 100 101"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                            fill="currentColor"
                        ></path>
                        <path
                            d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                            fill="currentFill"
                        ></path>
                    </svg>
                    <span class="sr-only">Loading...</span>
                </div>
            },
            false => view! { <div></div> }
        }
    }.into_view()
}

fn toggle_video_playback_action() -> Action<PlayerState, ()> {
    create_action(|input: &PlayerState| {
        let state = input.clone();
        async move { 
            state.toggle_playback().await;
        }
    })
}

fn provide_video_format_ctx(formats: &Formats) -> Result<(), RustyTubeError> {
    let webm_dash_formats = filter_webm_dash_formats(&formats.video_formats);

    let default_quality = expect_context::<RwSignal<Config>>().read_only().get().player.default_quality;
    let default_format = formats
        .video_formats
        .iter()
        .find(|x| x.quality_label == default_quality)
        .cloned();

    let format = match default_format {
        Some(format) => Ok(format),
        None => webm_dash_formats.last().cloned().ok_or(RustyTubeError::no_dash_format_available()),
    };
    provide_context(VideoFormatCtx(create_rw_signal(format.clone()?)));
    Ok(())
}

fn filter_webm_dash_formats(video_formats: &Vec<VideoFormat>) -> Vec<VideoFormat> {
    let mut webm_dash_formats = video_formats.clone();
    webm_dash_formats.retain(|format|
        match format.container.clone() {
            Some(container) => container == Container::WEBM,
            None => false,
        }
    );
    webm_dash_formats
}

fn cursor_visible() -> bool {
    !document().fullscreen() || controls_hovered()
}

fn controls_hovered() -> bool {
    let hovered_elements = document().query_selector_all(":hover").ok();

    match hovered_elements {
        Some(hovered_elements) => {
            let mut elements_vec: Vec<Element> = Vec::new();
            let mut index = 0;
            while let Some(node) = hovered_elements.item(index) {
                let element: Element = node.dyn_into().unwrap();
                elements_vec.push(element);
                index = index + 1;
            }
            elements_vec.iter().find(|element| element.id().eq(VIDEO_CONTROLS_ID)).is_some()
        },
        None => false,
    }
}

fn toggle_fullscreen() {
    match document().fullscreen() {
        true => document().exit_fullscreen(),
        false => { get_element_by_id::<HtmlDivElement>(VIDEO_CONTAINER_ID).unwrap().request_fullscreen(); }
    }
}

pub const VIDEO_CLASSES: &'static str = "
relative flex flex-col transition-all object-contain

data-[controls=false]:cursor-none

data-[fullwindow=false]:w-full
data-[fullwindow=false]:duration-300
data-[fullwindow=false]:ease-out

data-[fullwindow=true]:fullwindow
data-[fullwindow=true]:object-cover
data-[fullwindow=true]:ease-in
data-[fullwindow=true]:duration-300
";


















































