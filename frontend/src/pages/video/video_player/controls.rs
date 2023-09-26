use std::time::Duration;
use invidious::{VideoFormat, Caption};
use leptos::*;

use crate::contexts::{PlayerState, PlayerStyle, VIDEO_CONTROLS_ID};

use super::{control_btns::*};

#[component]
pub fn VideoPlayerControls(
    cx: Scope,
    formats: Vec<VideoFormat>,
    captions: Vec<Caption>,
) -> impl IntoView {
    let state = expect_context::<PlayerState>(cx);
    let style = expect_context::<PlayerStyle>(cx);

    let show_controls = move |_| { style.controls_visible.set(true) };

    view! {cx,
        <AnimatedShow
           when=style.controls_visible
           show_class="transition-all opacity-100 duration-1000"
           hide_class="transition-all opacity-0 duration-1000 cursor-none"
           hide_delay=Duration::from_millis(1000)
        >
            <div id=VIDEO_CONTROLS_ID on:mouseover=show_controls class="video-controls absolute bottom-0 left-0 flex flex-col w-full group/controls">
                <ProgressBar />
                <div class=VIDEO_CONTROLS>
                    <div class="flex flex-row">
                        <PauseBtn />
                        <VolumeKnob />
                        <TimeInfo />
                    </div>
                    <div class="flex flex-row">
                        <FormatDropdown formats=formats.clone() />
                        <FullWindowBtn />
                        <FullScreenBtn />
                    </div>
                </div>
            </div>
        </AnimatedShow>
    }
}

const VIDEO_CONTROLS: &'static str = "w-full bg-base-300 flex flex-row items-center justify-between rounded-b bg-opacity-50";