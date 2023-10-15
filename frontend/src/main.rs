#![feature(let_chains)]

mod components;
mod contexts;
mod icons;
mod pages;
mod utils;

use console_error_panic_hook;

use leptos::*;
use leptos_router::*;

use crate::{
    components::{Page, Sidebar},
    contexts::{provide_config_context_slices, provide_user_contexts, provide_user_resources},
    pages::{PopularSection, SearchSection, SubscriptionsSection, TrendingSection, VideoPage, ChannelPage},
};
use config::Config;

#[component]
fn App() -> impl IntoView {
    console_error_panic_hook::set_once();
    provide_config_context_slices(Config::load().unwrap_or_default());
    provide_user_contexts();
    provide_user_resources();

    let view = move || view! { <div></div> }.into_view();

    view! {
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Page/> }>
                    <Route path="/" view=move || view! { <TrendingSection/> }/>
                    <Route path="/player" view=move || view! { <VideoPage/> }/>
                    <Route path="/channel" view=move || view! { <ChannelPage/> }/>
                    <Route path="/subscriptions" view=move || view! { <SubscriptionsSection/> }/>
                    <Route path="/trending" view=move || view! { <TrendingSection/> }/>
                    <Route path="/popular" view=move || view! { <PopularSection/> }/>
                    <Route path="/search" view=move || view! { <SearchSection/> }/>
                    <Route path="/playlist" view=view/>
                    <Route path="/settings" view=view/>
                    <Route path="/about" view=view/>
                </Route>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}

