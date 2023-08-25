#![feature(let_chains)]

mod components;
mod icons;
mod pages;
mod contexts;

use console_error_panic_hook;

use leptos::*;
use leptos_router::*;

use config::Config;
use crate::contexts::provide_config_context_slices;


#[component]
fn App(cx: Scope) -> impl IntoView {
    console_error_panic_hook::set_once();
    provide_config_context_slices(cx, Config::load().unwrap_or_default());

    view! {cx,
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}