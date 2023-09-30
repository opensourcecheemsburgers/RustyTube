use leptos::*;
use utils::get_element_by_id;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

use crate::{contexts::VIDEO_CONTAINER_ID, icons::FullScreenIcon};

#[component]
pub fn FullScreenBtn() -> impl IntoView {
    let fullscreen = move |_| match document().fullscreen() {
        true => document().exit_fullscreen(),
        false => {
            get_element_by_id::<HtmlDivElement>(VIDEO_CONTAINER_ID)
                .unwrap()
                .request_fullscreen();
        }
    };

    view! {
        <button on:click=fullscreen class="btn btn-ghost btn-xs">
            <FullScreenIcon/>
        </button>
    }
}

