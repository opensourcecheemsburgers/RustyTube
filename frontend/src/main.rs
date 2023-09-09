#![feature(let_chains)]

mod components;
mod icons;
mod pages;
mod contexts;

use console_error_panic_hook;

use leptos::*;
use leptos_router::*;

use config::Config;
use crate::contexts::{provide_config_context_slices, provide_player_contexts, provide_user_contexts, provide_user_resources};
use crate::pages::{Homepage, VideoPage};


#[component]
fn App(cx: Scope) -> impl IntoView {
    console_error_panic_hook::set_once();
    provide_config_context_slices(cx, Config::load().unwrap_or_default());
    provide_user_contexts(cx);
    provide_user_resources(cx);
    provide_player_contexts(cx);

    let home = move |cx| view! { cx, <Homepage /> };
    let video = move |cx| view! { cx, <VideoPage /> };
    let view = move |cx| view! { cx, <div></div> }.into_view(cx);

    view! {cx,
        <Router>
            <Routes>
                <Route path="/" view=home />
                <Route path="/player" view=video />
                <Route path="/channel" view=view />
                <Route path="/subscriptions" view=view />
                <Route path="/playlist" view=view />
                <Route path="/settings" view=view />
                <Route path="/about" view=view />
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}