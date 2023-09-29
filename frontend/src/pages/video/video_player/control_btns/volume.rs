use std::ops::Mul;
use leptos::*;
use web_sys::{HtmlVideoElement, HtmlInputElement, Event, HtmlAudioElement};
use wasm_bindgen::JsCast;
use crate::contexts::{VolumeCtx, PlayerState, VIDEO_PLAYER_ID, AUDIO_PLAYER_ID};

use crate::icons::VolumeDefaultIcon;

#[component]
pub fn VolumeKnob() -> impl IntoView {
    let state = expect_context::<PlayerState>();
    let vol_position = state.volume.get() * 100f64  ;

    let vol_change = move |event| { change_volume(event, state.volume.write_only()) };

    view! {
        <div class="flex flex-row group items-center peer cursor-pointer">
            <button class="btn btn-ghost btn-xs peer" id="vol_btn">
                <VolumeDefaultIcon />
            </button>
            <input on:input=vol_change id="vol_knob" type="range" min="0" max="100" value=vol_position class=VOL_KNOB_CONTROLS />
        </div>
    }
}

fn change_volume(event: Event, volume: WriteSignal<f64>) {
    let video: HtmlVideoElement = document().get_element_by_id(VIDEO_PLAYER_ID).unwrap().dyn_into().unwrap();
    let audio: HtmlAudioElement = document().get_element_by_id(AUDIO_PLAYER_ID).unwrap().dyn_into().unwrap();

    let range: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
    let vol = range.value().parse::<f64>().unwrap_or_default() / 100f64;

    video.set_volume(vol);
    audio.set_volume(vol);
    volume.set(vol);
}

const VOL_KNOB_CONTROLS: &'static str = "
    w-0 overflow-hidden bg-transparent appearance-none transition-opacity opacity-0 ease-in duration-300 cursor-pointer

    peer-focus:block
    peer-focus:transition-opacity
    peer-focus:ease-in 
    peer-focus:duration-300
    peer-focus:w-32
    peer-focus:opacity-100

    peer-active:block
    peer-active:transition-opacity
    peer-active:ease-in 
    peer-active:duration-300
    peer-active:w-32
    peer-active:opacity-100

    hover:block
    hover:ease-in 
    hover:duration-300
    hover:w-32
    hover:opacity-100

    active:block
    active:ease-in 
    active:duration-300
    active:w-32
    active-focus:opacity-100

    [&::-webkit-slider-runnable-track]:rounded-full 
    [&::-webkit-slider-runnable-track]:bg-neutral-focus
    [&::-webkit-slider-thumb]:appearance-none 
    [&::-webkit-slider-thumb]:h-[8px] 
    [&::-webkit-slider-thumb]:w-[8px] 
    [&::-webkit-slider-thumb]:rounded-full 
    [&::-webkit-slider-thumb]:bg-primary
    [&::-ms-track]:rounded-full 
    [&::-ms-track]:bg-neutral-focus
    [&::-ms-thumb]:appearance-none
    [&::-ms-thumb]:h-[12px]
    [&::-ms-thumb]:w-[12px]
    [&::-ms-thumb]:rounded-full 
    [&::-ms-thumb]:bg-primary
    [&::-moz-range-track]:rounded-full 
    [&::-moz-range-track]:bg-neutral-focus
    [&::-moz-range-thumb]:appearance-none
    [&::-moz-range-thumb]:h-[8px]
    [&::-moz-range-thumb]:w-[8px]
    [&::-moz-range-thumb]:bg-primary
    [&::-moz-range-thumb]:border-primary
    [&::-moz-range-thumb]:rounded-full
";