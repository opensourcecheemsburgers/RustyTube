use invidious::Caption;
use leptos::*;
use std::time::Duration;

use crate::contexts::{PlayerState, PlayerStyle, VIDEO_CONTROLS_ID};

use super::control_btns::*;

#[component]
pub fn VideoPlayerControls(captions: Vec<Caption>) -> impl IntoView {
    let state = expect_context::<PlayerState>();
    let style = expect_context::<PlayerStyle>();

    let show_controls = move |_| style.controls_visible.set(true);

    view! {
        <AnimatedShow
            when=style.controls_visible
            show_class="transition-all opacity-100 duration-1000"
            hide_class="transition-all opacity-0 duration-1000 cursor-none"
            hide_delay=Duration::from_millis(1000)
        >
            <div
                id=VIDEO_CONTROLS_ID
                on:mouseover=show_controls
                class="absolute bottom-0 left-0 flex flex-col w-full group/controls"
            >
                <ProgressBar/>
                <div class="w-full bg-base-300 flex flex-row items-center justify-between rounded-b bg-opacity-50">
                    <div class="flex flex-row">
                        <PauseBtn/>
                        <VolumeKnob/>
                        <TimeInfo/>
                    </div>
                    <div class="flex flex-row">
                        <FormatDropdown/>
                        <FullWindowBtn/>
                        <FullScreenBtn/>
                    </div>
                </div>
            </div>
        </AnimatedShow>
    }
}

