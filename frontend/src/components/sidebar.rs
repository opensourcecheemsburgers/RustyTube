use leptos::*;
use leptos::svg::view;
use config::HomepageCategory;
use invidious::{Channel, Subscription, Subscriptions};
use rustytube_error::RustyTubeError;
use crate::contexts::{ServerCtx, SubscriptionsCtx, ThemeCtx, UiConfigCtx};
use crate::icons::{SubscriptionsIcon, TrendingIcon, PopularIcon, HamburgerIcon, FerrisIcon, CogIcon, HeartIcon, SettingsIcon};

#[component]
pub fn Sidebar(cx: Scope) -> impl IntoView {
	let expanded = create_rw_signal(cx, false.to_string());
	provide_context(cx, expanded);

	view! {cx,
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
pub fn SidebarSubs(cx: Scope) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>(cx);

	let subs_ctx = expect_context::<SubscriptionsCtx>(cx).0;
	let server_ctx = expect_context::<ServerCtx>(cx).0.0;

	let channels = create_resource(
		cx,
		move || (server_ctx.get(), subs_ctx.get()),
		|(server, subs)| async move {
			subs.fetch_channels(&server).await.unwrap()
		},
	);

	let channels_view = move || match channels.read(cx) {
		Some(mut results) => view! {cx, <SidebarSubsList results=results/>}.into_view(cx),
		None => view! {cx, <SidebarSubsListPlaceholderArray />}.into_view(cx)
	};


	view! {cx,
		<div class="min-h-[calc(100vh-308px)] max-h-[calc(100vh-308px)] overflow-y-scroll">
			{channels_view}
		</div>
	}
}

#[component]
pub fn SidebarHeader(cx: Scope) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>(cx);

	let toggle = move |_| expanded.set((!expanded.get().parse::<bool>().unwrap()).to_string());

	view! {cx,
		<button data-expanded=expanded on:click=toggle class=SIDEBAR_HEADER_CLASSES>
			<FerrisIcon />
			<p data-expanded=expanded class=SIDEBAR_HEADER_TEXT_CLASSES>{"RustyTube"}</p>
        </button>
	}
}

#[component]
pub fn ChannelButton(cx: Scope, channel: Channel) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>(cx);

	view! {cx,
		<div data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<img src=channel.thumbnails.first().unwrap().url.clone() class="w-6 h-6 rounded-full" />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>{channel.name}</p>
		</div>
	}
}

#[component]
pub fn SidebarSubsList(cx: Scope, results: Vec<Result<Channel, RustyTubeError>>) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>(cx);

	let mut channels_view_vec = Vec::new();

	let mut channels = Vec::new();
	results.into_iter().for_each(|channel| {
		if let Ok(channel) = channel {
			channels.push(channel);
		}
	});
	channels.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

	channels.into_iter().for_each(|channel| {
		let view = view! {cx, <ChannelButton channel=channel />};
		channels_view_vec.push(view);
	});
	channels_view_vec.collect_view(cx)
}

#[component]
pub fn SidebarSubsListPlaceholderArray(cx: Scope) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>(cx);

	let mut channels = Subscriptions::load().unwrap_or_default().channels;
	channels.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
	channels.into_iter().map(|channel| {
		view! {cx,
		<div data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<div class="w-6 h-6 rounded-full animate-pulse bg-neutral" />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>{channel.name}</p>
		</div>
	}
	}).collect_view(cx)
}

#[component]
pub fn SubscriptionsButton(cx: Scope) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>(cx);

	let change_category = move |_| { change_homepage_category(cx, HomepageCategory::Subscriptions)};

	view! {cx,
		<div on:click=change_category data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<SubscriptionsIcon />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>Subscriptions</p>
		</div>
	}
}

#[component]
pub fn TrendingButton(cx: Scope) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>(cx);

	let change_category = move |_| { change_homepage_category(cx, HomepageCategory::Trending)};

	view! {cx,
		<div on:click=change_category data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<TrendingIcon />
		<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>Trending</p>
		</div>
	}
}

#[component]
pub fn PopularButton(cx: Scope) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>(cx);

	let change_category = move |_| { change_homepage_category(cx, HomepageCategory::Popular)};

	view! {cx,
		<div on:click=change_category data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<PopularIcon />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>Popular</p>
		</div>
	}
}

#[component]
pub fn SettingsButton(cx: Scope) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>(cx);

	view! {cx,
		<div data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<SettingsIcon />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>Settings</p>
		</div>
	}
}

#[component]
pub fn DonateButton(cx: Scope) -> impl IntoView {
	let expanded = expect_context::<RwSignal<String>>(cx);

	view! {cx,
		<div data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<HeartIcon />
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>Contribute</p>
		</div>
	}
}

fn change_homepage_category(cx: Scope, category: HomepageCategory) {
	let ui_config_ctx = expect_context::<UiConfigCtx>(cx);
	let mut ui_config = ui_config_ctx.0.0.get();
	ui_config.homepage = category;
	ui_config_ctx.0.1.set(ui_config);
}

pub const SIDEBAR_CLASSES: &'static str = "
flex flex-col min-h-screen max-h-screen bg-base-200 transition-all duration-300 overflow-x-visible overflow-y-hidden

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