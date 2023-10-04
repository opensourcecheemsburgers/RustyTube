use gloo::console::debug;
use leptos::*;
use invidious::{AudioFormat, Container, LegacyFormat, VideoFormat};
use crate::contexts::{VideoFormatCtx, PlayerState};

use crate::icons::CogIcon;

#[component]
pub fn FormatDropdown(formats: Vec<VideoFormat>) -> impl IntoView {
	view! {
		<div class="dropdown dropdown-top dropdown-end z-20">
			<DropdownBtn/>
			<DropdownContent formats=formats/>
		</div>
	}
}

#[component]
pub fn DropdownBtn() -> impl IntoView {
    view! {
		<label tabindex="0" class="btn btn-ghost btn-xs">
			<CogIcon/>
		</label>
	}
}

#[component]
pub fn DropdownContent(formats: Vec<VideoFormat>) -> impl IntoView {
    view! {
		<ul
			tabindex="0"
			class="menu dropdown-content mb-4 px-1.5 py-3 shadow bg-base-200 rounded-xl w-max h-max"
		>
			<VideoFormatList formats=formats/>
		</ul>
	}
}

#[component]
pub fn ListItem(children: Children) -> impl IntoView {
    view! { <button class="btn btn-sm lowercase btn-ghost">{children()}</button> }
}

#[component]
pub fn VideoFormatList(formats: Vec<VideoFormat>) -> impl IntoView {
    view! {
		<div class="flex flex-col bg-base-200">

			{formats
				.into_iter()
				.map(|format| {
					view! { <VideoFormatListItem format=format/> }
				})
				.collect_view()}

		</div>
	}
}

#[component]
pub fn VideoFormatListItem(format: VideoFormat) -> impl IntoView {
    let quality_label = format.quality_label.to_string();

    let change_format = create_action(|input: &(PlayerState, VideoFormat)| {
        let input = input.clone();
        async move {
            input.0.change_quality(input.1).await;
        }
    });

    let state = expect_context::<PlayerState>();
    let on_click = move |_| { change_format.dispatch((state, format.clone())) };

    view! {
		<div on:click=on_click>
			<ListItem>{quality_label}</ListItem>
		</div>
	}
}


