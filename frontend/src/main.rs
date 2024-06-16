#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en-US");

mod components;
mod contexts;
mod icons;
mod pages;
mod resources;
mod themes;
mod utils;

use config::Config;

use leptos::{component, mount_to_body, provide_context, view, IntoView};
use leptos_router::{Route, Router, Routes};
pub use themes::*;

use crate::{
	components::Page,
	contexts::{provide_config_context_slices, provide_toaster_ctx},
	pages::{
		ChannelPage, PlaylistsSection, PopularSection, SearchSection,
		SettingsPage, SubscriptionsSection, TrendingSection, VideoPage,
	},
	resources::{
		InstancesResource, PlaylistsCtx, SponsorBlockResource,
		SubscriptionsCtx, SubscriptionsThumbnailsResource,
		SubscriptionsVideosResource,
	},
};

#[component]
fn App() -> impl IntoView {
	console_error_panic_hook::set_once();
	provide_toaster_ctx();
	provide_context::<SponsorBlockResource>(SponsorBlockResource::empty());

	provide_config_context_slices(Config::load().unwrap_or_default());

	let subscriptions = SubscriptionsCtx::initialise();
	provide_context(subscriptions);
	provide_context(SubscriptionsVideosResource::initialise(subscriptions));
	provide_context(SubscriptionsThumbnailsResource::initialise(subscriptions));
	provide_context(InstancesResource::initialise());
	provide_context(PlaylistsCtx::initialise());

	view! {
		<Router>
			<Routes>
				<Route path="" view=move || view! { <Page/> }>
					<Route path="/" view=move || view! { <TrendingSection/> }/>
					<Route path="/player" view=move || view! { <VideoPage/> }/>
					<Route
						path="/channel"
						view=move || view! { <ChannelPage/> }
					/>
					<Route
						path="/subscriptions"
						view=move || view! { <SubscriptionsSection/> }
					/>
					<Route
						path="/trending"
						view=move || view! { <TrendingSection/> }
					/>
					<Route
						path="/popular"
						view=move || view! { <PopularSection/> }
					/>
					<Route
						path="/search"
						view=move || view! { <SearchSection/> }
					/>
					<Route
						path="/playlist"
						view=move || view! { <VideoPage/> }
					/>
					<Route
						path="/playlists"
						view=move || view! { <PlaylistsSection/> }
					/>
					<Route
						path="/settings"
						view=move || view! { <SettingsPage/> }
					/>
					<Route path="/about" view=move || ().into_view()/>
				</Route>
			</Routes>
		</Router>
	}
}

fn main() {
	mount_to_body(|| view! { <App/> });
}
