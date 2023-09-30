use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, HtmlProgressElement, HtmlVideoElement, HtmlAudioElement, DragEvent};
use utils::get_element_by_id;

use crate::contexts::{PlayerState, PlayerStyle};

#[component]
pub fn ProgressBar() -> impl IntoView {
    let state = expect_context::<PlayerState>();
    let style = expect_context::<PlayerStyle>();

    let seek = create_action(move |input: &(PlayerState, f64)| {
        let input = input.clone();
        async move { input.0.seek(input.1).await }
    });

    let on_click = move |mouse_event| {
        let state = state.clone();
        let time = seek_pos(mouse_event);
        seek.dispatch((state, time));
    };
    let on_drag = move |drag_event: DragEvent| {
        let state = state.clone();
        let time = seek_drag_pos(drag_event);
        seek.dispatch((state, time));
    };

    view! {
        <progress
            on:click=on_click
            on:dragend=on_drag
            data-controlsvisible=style.controls_visible
            max=state.duration.read_only()
            value=state.current_time.read_only()
            class=PROGRESS_BAR
        ></progress>
    }
}

fn seek_pos(mouse_event: MouseEvent) -> f64 {
    let progress: HtmlProgressElement = mouse_event.target().unwrap().dyn_into().unwrap();
    let x = mouse_event.offset_x() as f64;
    let offset_width = progress.offset_width() as f64;
    let max = progress.max();

    x * max / offset_width
}

fn seek_drag_pos(drag_event: DragEvent) -> f64 {
    let progress: HtmlProgressElement = drag_event.target().unwrap().dyn_into().unwrap();
    let x = drag_event.offset_x() as f64;
    let offset_width = progress.offset_width() as f64;
    let max = progress.max();

    x * max / offset_width
}

const PROGRESS_BAR: &'static str = "
    peer/progress z-10

    progress progress-primary bg-neutral opacity-100 h-1 w-full cursor-pointer
    
    transition-all ease-in duration-300

    hover:ease-in 
    hover:duration-300
    hover:opacity-100
    hover:mt-0
    hover:h-1
    hover:scale-y-[2.5]

    active:ease-in 
    active:duration-300
    active-focus:opacity-100
    active:mt-0
    active:rounded-none
    active:h-1
    active:scale-y-[2.5]

    focus:ease-in 
    focus:duration-300
    focus-focus:opacity-100
    focus:mt-0
    focus:rounded-none
    focus:h-1
    focus:scale-y-[2.5]

    peer-active/controls:ease-in 
    peer-active/controls:duration-300
    peer-active/controls:opacity-100
    peer-active/controls:mt-0
    peer-active/controls:rounded-none
    peer-active/controls:h-1
    peer-active/controls:scale-y-[2.5]

    peer-hover/controls:ease-in 
    peer-hover/controls:duration-300
    peer-hover/controls:opacity-100
    peer-hover/controls:mt-0
    peer-hover/controls:rounded-none
    peer-hover/controls:h-1
    peer-hover/controls:scale-y-[2.5]

    peer-focus/controls:ease-in 
    peer-focus/controls:duration-300
    peer-focus/controls:opacity-100
    peer-focus/controls:mt-0
    peer-focus/controls:rounded-none
    peer-focus/controls:h-1
    peer-focus/controls:scale-y-[2.5]
";


