use gloo::console::debug;
use leptos::*;
use invidious::{AudioFormat, Container, LegacyFormat, VideoFormat};
use crate::contexts::{VideoFormatCtx, PlayerState};

use crate::icons::CogIcon;

#[component]
pub fn FormatDropdown(cx: Scope, formats: Vec<VideoFormat>) -> impl IntoView {
	view! {cx,
        <div class="dropdown dropdown-top dropdown-end z-20">
            <DropdownBtn />
            <DropdownContent formats=formats />
        </div>
    }
}

#[component]
pub fn DropdownBtn(cx: Scope) -> impl IntoView {
    view! {cx,
        <label tabindex="0" class="btn btn-ghost btn-xs">
            <CogIcon />
        </label>
    }
}

#[component]
pub fn DropdownContent(cx: Scope, formats: Vec<VideoFormat>) -> impl IntoView {
    view! {cx,
        <ul tabindex="0" class="menu dropdown-content mb-4 px-1.5 py-3 shadow bg-base-200 rounded-xl w-max h-max">
                <VideoFormatList formats=formats />
        </ul>
    }
}

#[component]
pub fn ListItem(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <button class="btn btn-sm lowercase btn-ghost">
            {children(cx)}
        </button>
    }
}

#[component]
pub fn VideoFormatList(cx: Scope, formats: Vec<VideoFormat>) -> impl IntoView {
    view! {cx,
        <div class="flex flex-col bg-base-200">
        {
            formats.into_iter().map(|format| {
                view! {cx, <VideoFormatListItem format=format/>}
            }).collect_view(cx)
        }
        </div>
    }
}

#[component]
pub fn VideoFormatListItem(cx: Scope, format: VideoFormat) -> impl IntoView {
    let video_format = expect_context::<VideoFormatCtx>(cx);
    let quality_label = format.quality_label.to_string();

    let change_format = create_action(cx, |input: &(PlayerState, VideoFormat)| {
        let input = input.clone();
        async move {
            input.0.change_quality(input.1).await;
        }
    });

    let state = expect_context::<PlayerState>(cx);
    let on_click = move |_| { change_format.dispatch((state, format.clone())) };

    view! {cx,
        <div on:click=on_click>
            <ListItem>
                { quality_label }
            </ListItem>
        </div>
    }
}