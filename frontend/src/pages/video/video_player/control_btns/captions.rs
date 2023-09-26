use invidious::Caption;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlVideoElement, MouseEvent, TextTrack, TextTrackMode};

use crate::{icons::CaptionsIcon, contexts::VIDEO_PLAYER_ID};

#[component]
pub fn CaptionsDropdown(cx: Scope, captions: Vec<Caption>) -> impl IntoView {
    view! {cx, 
        <div class="dropdown dropdown-top dropdown-end z-20">
            <CaptionsDropdownBtn />
            <CaptionsDropdownContent captions=captions />
        </div>
    }
}

#[component]
pub fn CaptionsDropdownBtn(cx: Scope) -> impl IntoView {
    view! {cx,
        <label tabindex="0" class="btn btn-ghost btn-xs">
            <CaptionsIcon />
        </label>
    }
}

#[component]
pub fn CaptionsDropdownContent(cx: Scope, captions: Vec<Caption>) -> impl IntoView {
    view! {cx,
        <ul tabindex="0" class="menu dropdown-content py-3 mb-4 mr-2 shadow bg-base-300 rounded-xl w-max h-auto max-h-48">
            <div class="flex flex-col h-full overflow-y-scroll space-y-2 px-3"> {
                captions.into_iter().map(|caption| {
                    view! {cx, <CaptionsDropdownItem caption=caption />}
                }).collect_view(cx)
            }
            </div>
        </ul>
    }
}

#[component]
pub fn CaptionsDropdownItem(cx: Scope, caption: Caption) -> impl IntoView {
    let set_captions = move |_ :MouseEvent| {
        let video: HtmlVideoElement = document().get_element_by_id(VIDEO_PLAYER_ID).unwrap().dyn_into().unwrap();
        video.text_tracks().unwrap().get_track_by_id(&caption.language).unwrap().set_mode(TextTrackMode::Showing);
    };

    view! {cx,
        <div class="p-3 rounded-lg bg-base-100 cursor-pointer">
            <a on:click=set_captions class="text-base-content font-sans">
                { caption.label }
            </a>
        </div>
    }
}