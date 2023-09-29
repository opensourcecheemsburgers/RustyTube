use leptos::*;
use leptos::svg::view;
use config::HomepageCategory;
use invidious::{Channel, Subscription, Subscriptions};
use rustytube_error::RustyTubeError;
use crate::contexts::{ChannelsCtx, ServerCtx, SubscriptionsCtx, ThemeCtx, UiConfigCtx};
use crate::icons::{SubscriptionsIcon, TrendingIcon, PopularIcon, HamburgerIcon, FerrisIcon, CogIcon, HeartIcon, SettingsIcon};

#[component]
pub fn Sidebar() -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>();

	view!{
        <div
			data-expanded=expanded
			class=SIDEBAR_CLASSES
		>
			<SidebarHeader />
			<div>
				<div class="border-b-[1px] border-b-primary">
					<SubscriptionsButton />
					<TrendingButton />
					<PopularButton />
				</div>
				<SidebarSubs />
				<div class="border-t-[1px] border-t-primary">
					<SettingsButton />
					<DonateButton />
				</div>
			</div>
        </div>
    }
}

#[component]
pub fn SidebarSubs() -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>();
	let channels = expect_context::<ChannelsCtx>().0;
	let channels_view = move || match channels.get() {
		Some(mut results) => view!{<SidebarSubsList results=results/>}.into_view(),
		None => view!{<SidebarSubsListPlaceholderArray />}.into_view()
	};

	view! {
		<div class="min-h-[calc(100vh-19.25rem)] max-h-[calc(100vh-19.25rem)] overflow-x-auto overflow-y-scroll scroll-smooth">
			{channels_view}
		</div>
	}
}

#[component]
pub fn SidebarHeader() -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>();

	let toggle = move |_| expanded.set((!expanded.get().parse::<bool>().unwrap()).to_string());

	view!{
		<button data-expanded=expanded on:click=toggle class=SIDEBAR_HEADER_CLASSES>
			<FerrisIcon />
			<p data-expanded=expanded class=SIDEBAR_HEADER_TEXT_CLASSES>{"RustyTube"}</p>
        </button>
	}
}

#[component]
pub fn ChannelButton(channel: Channel) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>();

	view!{
		<div data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<img src=channel.thumbnails.first().unwrap().url.clone() class="w-6 h-6 rounded-full" />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>{channel.name}</p>
		</div>
	}
}

#[component]
pub fn SidebarSubsList(results: Vec<Result<Channel, RustyTubeError>>) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>();

	let mut channels_view_vec = Vec::new();

	let mut channels = Vec::new();
	results.into_iter().for_each(|channel| {
		if let Ok(channel) = channel {
			channels.push(channel);
		}
	});
	channels.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

	channels.into_iter().for_each(|channel| {
		let view = view!{<ChannelButton channel=channel />};
		channels_view_vec.push(view);
	});
	channels_view_vec.collect_view()
}

#[component]
pub fn SidebarSubsListPlaceholderArray() -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>();

	let mut channels = Subscriptions::load().unwrap_or_default().channels;
	channels.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
	channels.into_iter().map(|channel| {
		view!{
		<div data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<div class="w-6 h-6 rounded-full animate-pulse bg-neutral" />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>{channel.name}</p>
		</div>
	}
	}).collect_view()
}

#[component]
pub fn SubscriptionsButton() -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>();

	let change_category = move |_| { change_homepage_category(HomepageCategory::Subscriptions)};

	view!{
		<div on:click=change_category data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<SubscriptionsIcon />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>Subscriptions</p>
		</div>
	}
}

#[component]
pub fn TrendingButton() -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>();

	let change_category = move |_| { change_homepage_category(HomepageCategory::Trending)};

	view!{
		<div on:click=change_category data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<TrendingIcon />
		<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>Trending</p>
		</div>
	}
}

#[component]
pub fn PopularButton() -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>();

	let change_category = move |_| { change_homepage_category(HomepageCategory::Popular)};

	view!{
		<div on:click=change_category data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<PopularIcon />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>Popular</p>
		</div>
	}
}

#[component]
pub fn SettingsButton() -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>();

	view!{
		<div data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<SettingsIcon />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>Settings</p>
		</div>
	}
}

#[component]
pub fn DonateButton() -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>();

	view!{
		<div data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<HeartIcon />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>Contribute</p>
		</div>
	}
}

fn change_homepage_category(category: HomepageCategory) {
	let ui_config_ctx = expect_context::<UiConfigCtx>();
	let mut ui_config = ui_config_ctx.0.0.get();
	ui_config.homepage = category;
	ui_config_ctx.0.1.set(ui_config);
}

pub const SIDEBAR_CLASSES: &'static str = "
flex flex-col min-h-screen max-h-screen bg-base-200 transition-all duration-300

data-[expanded=false]:w-16
data-[expanded=true]:w-64
";


pub const SIDEBAR_HEADER_CLASSES: &'static str = "
btn btn-ghost h-16 transition-all duration-300 flex flex-row flex-nowrap space-x-0 overflow-hidden

data-[expanded=false]:items-center
data-[expanded=false]:justify-center
data-[expanded=true]:items-center
data-[expanded=true]:justify-start
";

pub const SIDEBAR_HEADER_TEXT_CLASSES: &'static str = "
normal-case font-display font-medium text-2xl

data-[expanded=false]:opacity-0
data-[expanded=false]:absolute

data-[expanded=false]:static
data-[expanded=true]:opacity-100
data-[expanded=true]:transition-opacity
data-[expanded=true]:duration-1000
";

pub const SIDEBAR_ITEM_CLASSES: &'static str = "
btn btn-ghost transition-all duration-300 flex flex-row flex-nowrap space-x-2

data-[expanded=false]:items-center
data-[expanded=false]:justify-center

data-[expanded=true]:items-center
data-[expanded=true]:justify-start
";

pub const SIDEBAR_ITEM_TEXT_CLASSES: &'static str = "
normal-case font-sans font-normal

data-[expanded=false]:opacity-0
data-[expanded=false]:hidden
data-[expanded=false]:h-0
data-[expanded=false]:w-0

data-[expanded=false]:flex
data-[expanded=true]:opacity-100
data-[expanded=true]:transition-opacity
data-[expanded=true]:duration-1000
";