#![feature(let_chains)]

mod components;
mod contexts;
mod icons;
mod pages;
mod themes;
mod utils;

use config::Config;
use console_error_panic_hook;
use leptos::*;
use leptos_router::*;
pub use themes::*;

use crate::{
	components::Page,
	contexts::{provide_config_context_slices, provide_user_contexts, provide_user_resources},
	pages::{
		ChannelPage, PopularSection, SearchSection, SettingsPage, SubscriptionsSection,
		TrendingSection, VideoPage,
	},
};

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
					<Route path="/settings" view=move || view! { <SettingsPage/> }/>
					<Route path="/about" view=view/>
				</Route>
			</Routes>
		</Router>
	}
}

fn main() {
	mount_to_body(|| view! { <App/> })
}
