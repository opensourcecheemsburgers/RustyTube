#![feature(let_chains)]

mod components;
mod contexts;
mod icons;
mod pages;

use console_error_panic_hook;

use leptos::*;
use leptos_router::*;

use crate::contexts::{
    provide_config_context_slices, provide_player_contexts, provide_user_contexts,
    provide_user_resources,
};
use crate::pages::{Homepage, VideoPage};
use config::Config;

#[component]
fn App() -> impl IntoView {
    console_error_panic_hook::set_once();
    provide_config_context_slices(Config::load().unwrap_or_default());
    provide_user_contexts();
    provide_user_resources();
    provide_player_contexts();

    let home = move || view! { <Homepage/> };
    let video = move || view! { <VideoPage/> };
    let view = move || view! { <div></div> }.into_view();

    view! {
        <Router>
            <Routes>
                <Route path="/" view=home/>
                <Route path="/player" view=video/>
                <Route path="/channel" view=view/>
                <Route path="/subscriptions" view=view/>
                <Route path="/playlist" view=view/>
                <Route path="/settings" view=view/>
                <Route path="/about" view=view/>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}

