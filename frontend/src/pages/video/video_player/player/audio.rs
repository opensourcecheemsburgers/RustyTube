use invidious::{AudioFormat, Container};
use leptos::{html::audio, *};

use crate::{
    contexts::{PlayerState, AUDIO_PLAYER_ID},
    utils::is_webkit,
};

#[component]
pub fn AudioStream(formats: Vec<AudioFormat>) -> impl IntoView {
    let state = expect_context::<PlayerState>();

    // let sources = filter_mp4_only(&formats)
    //     .into_iter()
    //     .map(|format| view! { <source src=format.url.clone()/> })
    //     .collect_view();

    let source = filter_mp4_only(&formats)
        .first()
        .map(|format| format.url.clone());

    view! {
        <audio
            on:waiting=move |_| {
                state.set_audio_ready(false);
            }

            on:loadedmetadata=move |_| {
                if is_webkit() {
                    state.set_audio_ready(true);
                }
            }

            on:canplay=move |_| {
                state.set_audio_ready(true);
            }

            id=AUDIO_PLAYER_ID
            preload="auto"
            controls=false
            autoplay=false
            playsinline=true
            src=source
        ></audio>
    }
}

fn filter_mp4_only(formats: &Vec<AudioFormat>) -> Vec<AudioFormat> {
    let formats = formats.clone();
    formats
        .into_iter()
        .filter_map(|format| {
            format
                .clone()
                .container
                .map(|container| container.eq(&Container::M4A).then(|| format))
                .flatten()
        })
        .collect::<Vec<AudioFormat>>()
}

