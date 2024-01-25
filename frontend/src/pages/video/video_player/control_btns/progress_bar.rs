use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, HtmlProgressElement, MouseEvent};

use crate::contexts::{PlayerState, PlayerStyle};

#[component]
pub fn ProgressBar() -> impl IntoView {
	let state = expect_context::<PlayerState>();
	let style = expect_context::<PlayerStyle>();

	let on_click = move |mouse_event| {
		let _ = state.seek(seek_pos(mouse_event));
	};
	let on_drag = move |drag_event: DragEvent| {
		let _ = state.seek(seek_pos(drag_event));
	};

	let tip_styles = RwSignal::new("bottom: 48px; left: 0px;".to_string());
	let tip_classes = RwSignal::new("hidden".to_string());
	let tip_time = RwSignal::new("0:00".to_string());

	let open_tip = move |mouse_event: MouseEvent| {
		tip_time.set(get_seek_pos_as_time_str(mouse_event.clone()));
		let styles = format!("bottom: 48px; left: {}px;", mouse_event.offset_x());
		tip_styles.set(styles);
		tip_classes.set(
			"absolute p-2 rounded-lg z-100 bg-primary text-primary-content h-fit w-fit".to_string(),
		);
	};
	let close_tip = move |_| {
		tip_classes.set("hidden".to_string());
	};

	view! {
		<div style=tip_styles class=tip_classes>
			{tip_time}
		</div>
		<progress
			on:mouseover=open_tip
			on:mousemove=open_tip
			on:mouseout=close_tip
			on:click=on_click
			on:dragend=on_drag
			data-controlsvisible=style.controls_visible
			max=state.duration.read_only()
			value=state.current_time.read_only()
			class=PROGRESS_BAR
		></progress>
	}
}

fn get_seek_pos_as_time_str<E>(event: E) -> String
where
	E: AsRef<MouseEvent>,
{
	let seek_pos = seek_pos(event);
	utils::unix_to_hours_secs_mins(seek_pos)
}

fn seek_pos<E>(event: E) -> f64
where
	E: AsRef<MouseEvent>,
{
	let progress: HtmlProgressElement = event.as_ref().target().unwrap().dyn_into().unwrap();
	let x = event.as_ref().offset_x() as f64;
	let offset_width = progress.offset_width() as f64;
	let max = progress.max();

	x * max / offset_width
}

const PROGRESS_BAR: &'static str = "
    peer/progress z-10

    progress progress-primary bg-neutral opacity-100 h-2.5 w-full cursor-pointer
    
    transition-all ease-in duration-300

    hover:ease-in 
    hover:duration-300
    hover:opacity-100
    hover:mt-0
    hover:h-3.5

    active:ease-in 
    active:duration-300
    active-focus:opacity-100
    active:mt-0
    active:rounded-none
    active:h-3.5

    focus:ease-in 
    focus:duration-300
    focus-focus:opacity-100
    focus:mt-0
    focus:rounded-none
    focus:h-3.5

    peer-active/controls:ease-in 
    peer-active/controls:duration-300
    peer-active/controls:opacity-100
    peer-active/controls:mt-0
    peer-active/controls:rounded-none
    peer-active/controls:h-3.5

    peer-hover/controls:ease-in 
    peer-hover/controls:duration-300
    peer-hover/controls:opacity-100
    peer-hover/controls:mt-0
    peer-hover/controls:rounded-none
    peer-hover/controls:h-3.5

    peer-focus/controls:ease-in 
    peer-focus/controls:duration-300
    peer-focus/controls:opacity-100
    peer-focus/controls:mt-0
    peer-focus/controls:rounded-none
    peer-focus/controls:h-3.5
";
