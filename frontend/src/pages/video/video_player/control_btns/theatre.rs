use leptos::*;
use crate::{icons::FullWindowIcon, contexts::PlayerStyle};

#[component]
pub fn FullWindowBtn(cx: Scope) -> impl IntoView {
    let style = expect_context::<PlayerStyle>(cx);
    let toggle_fullwindow = move |_| { style.full_window.set(style.full_window.get()) };

    view! {cx, 
        <button on:click=toggle_fullwindow class="btn btn-ghost btn-xs">
            <FullWindowIcon />
        </button>
    }
}