use invidious::{ChannelThumb, Subscription, Subscriptions};
use leptos::*;
use rustytube_error::RustyTubeError;
use utils::get_element_by_id;
use web_sys::HtmlDialogElement;

use crate::{
	components::{
		donate_modal::{DonateModal, DONATE_MODAL_ID},
		FerrisError,
	},
	contexts::ChannelThumbsCtx,
	icons::*,
};

#[derive(Clone, Copy)]
pub struct ExpandedCtx(RwSignal<String>);

#[component]
pub fn Sidebar() -> impl IntoView {
	let expanded = create_rw_signal(false.to_string());
	provide_context(ExpandedCtx(expanded));

	view! {
		<div data-expanded=expanded class=SIDEBAR_CLASSES>
			<SidebarHeader/>
			<div>
				<div class="border-b-[1px] border-b-primary">
					<SubscriptionsButton/>
					<TrendingButton/>
					<PopularButton/>
				</div>
				<Subs/>
				<div class="border-t-[1px] border-t-primary">
					<SettingsButton/>
					<DonateButton/>
				</div>
			</div>
		</div>
		<DonateModal/>
	}
}

#[component]
fn SidebarHeader() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;
	let is_open = move || expanded.get().parse::<bool>().unwrap_or_default();
	let toggle = move |_| expanded.set((!is_open()).to_string());

	view! {
		<button
			data-tip="Open drawer"
			data-expanded=expanded
			on:click=toggle
			class=SIDEBAR_HEADER_CLASSES
		>
			<FerrisIcon/>
			<p data-expanded=expanded class=SIDEBAR_HEADER_TEXT_CLASSES>
				{"RustyTube"}
			</p>
		</button>
	}
}

#[component]
fn SubscriptionsButton() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	let go_to_subs = move |_| {
		let navigate = leptos_router::use_navigate();
		request_animation_frame(move || {
			_ = navigate("/subscriptions", Default::default());
		})
	};

	view! {
		<div data-expanded=expanded data-tip="Subscriptions" class=SIDEBAR_TOOLTIP_CLASSES>
			<button on:click=go_to_subs data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
				<SubscriptionsIcon/>
				<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
					Subscriptions
				</p>
			</button>
		</div>
	}
}

#[component]
fn TrendingButton() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	let go_to_trending = move |_| {
		let navigate = leptos_router::use_navigate();
		request_animation_frame(move || {
			_ = navigate("/trending", Default::default());
		})
	};

	view! {
		<div data-expanded=expanded data-tip="Trending" class=SIDEBAR_TOOLTIP_CLASSES>
			<button on:click=go_to_trending data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
				<TrendingIcon/>
				<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
					Trending
				</p>
			</button>
		</div>
	}
}

#[component]
fn PopularButton() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	let go_to_popular = move |_| {
		let navigate = leptos_router::use_navigate();
		request_animation_frame(move || {
			_ = navigate("/popular", Default::default());
		})
	};

	view! {
		<div data-expanded=expanded data-tip="Popular" class=SIDEBAR_TOOLTIP_CLASSES>
			<button on:click=go_to_popular data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
				<PopularIcon/>
				<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
					Popular
				</p>
			</button>
		</div>
	}
}

#[component]
fn Subs() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;
	let channel_thumbs_ctx = expect_context::<ChannelThumbsCtx>().0;
	view! {
		<div data-expanded=expanded class=SIDEBAR_SUBS_CLASSES>
			<Suspense fallback=move || {
				view! { <SubsPlaceholders/> }
			}>
				{move || {
					channel_thumbs_ctx
						.get()
						.map(|channel_thumbs| match channel_thumbs {
							Ok(channel_thumbs) => view! { <SubsInner results=channel_thumbs/> },
							Err(err) => view! { <FerrisError error=err/> },
						})
				}}

			</Suspense>
		</div>
	}
}

#[component]
pub fn SubsInner(results: Vec<Result<ChannelThumb, RustyTubeError>>) -> impl IntoView {
	let mut channels =
		results.into_iter().filter_map(|res| res.ok()).collect::<Vec<ChannelThumb>>();
	channels.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

	channels.into_iter().map(|channel| view! { <ChannelButton channel=channel/> }).collect_view()
}

#[component]
fn SubsPlaceholders() -> impl IntoView {
	let mut subscriptions = Subscriptions::load().unwrap_or_default().channels;
	subscriptions.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
	subscriptions
		.into_iter()
		.map(|sub| {
			view! { <ChannelButtonPlaceholder sub=sub/> }
		})
		.collect_view()
}

#[component]
pub fn ChannelButton(channel: ChannelThumb) -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;
	let thumb_url = channel.thumbnails.first().map(|thumb| thumb.url.clone());

	let go_to_channel_page = move |_| {
		let navigate = leptos_router::use_navigate();
		let author_id = channel.id.clone();
		request_animation_frame(move || {
			_ = navigate(&format!("/channel?id={}", author_id), Default::default());
		})
	};

	view! {
		<button on:click=go_to_channel_page data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<img src=thumb_url class="w-6 h-6 rounded-full"/>
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
				{channel.name}
			</p>
		</button>
	}
}

#[component]
fn ChannelButtonPlaceholder(sub: Subscription) -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	view! {
		<div data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
			<div class="w-6 h-6 rounded-full animate-pulse bg-neutral"></div>
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
				{sub.name}
			</p>
		</div>
	}
}

#[component]
fn SettingsButton() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	let go_to_settings = move |_| {
		let navigate = leptos_router::use_navigate();
		request_animation_frame(move || {
			_ = navigate("/settings", Default::default());
		})
	};

	view! {
		<div
			on:click=go_to_settings
			data-expanded=expanded
			data-tip="Settings"
			class=SIDEBAR_TOOLTIP_CLASSES
		>
			<button data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
				<SettingsIcon/>
				<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
					Settings
				</p>
			</button>
		</div>
	}
}

#[component]
fn DonateButton() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	let open_donate_modal = move |_| {
		let modal = get_element_by_id::<HtmlDialogElement>(DONATE_MODAL_ID);
		if let Ok(modal) = modal {
			modal.set_open(true);
		}
	};

	view! {
		<div data-expanded=expanded data-tip="Donate" class=SIDEBAR_TOOLTIP_CLASSES>
			<button on:click=open_donate_modal data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
				<HeartIcon/>
				<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
					Contribute
				</p>
			</button>
		</div>
	}
}

pub const SIDEBAR_CLASSES: &'static str = "
flex flex-col min-h-screen max-h-screen bg-base-200 transition-all duration-300

data-[expanded=false]:w-16
data-[expanded=true]:w-64
";

pub const SIDEBAR_HEADER_CLASSES: &'static str = "
btn btn-ghost h-16 transition-all duration-300 flex flex-row flex-nowrap space-x-0 overflow-hidden

data-[expanded=false]:w-16
data-[expanded=true]:w-64

data-[expanded=false]:items-center
data-[expanded=false]:justify-center
data-[expanded=true]:items-center
data-[expanded=true]:justify-start
";

pub const SIDEBAR_HEADER_TEXT_CLASSES: &'static str = "
normal-case font-display font-medium text-2xl

data-[expanded=false]:hidden
data-[expanded=false]:opacity-0
data-[expanded=false]:absolute

data-[expanded=false]:static
data-[expanded=true]:opacity-100
data-[expanded=true]:transition-opacity
data-[expanded=true]:duration-1000
";

pub const SIDEBAR_TOOLTIP_CLASSES: &'static str = "
data-[expanded=false]:tooltip
data-[expanded=false]:tooltip-primary
data-[expanded=false]:tooltip-right
";

pub const SIDEBAR_ITEM_CLASSES: &'static str = "
btn btn-ghost transition-all duration-300 flex flex-row flex-nowrap space-x-2

data-[expanded=false]:w-16
data-[expanded=true]:w-64

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
pub const SIDEBAR_SUBS_CLASSES: &'static str = "
min-h-[calc(100vh-19.25rem)] max-h-[calc(100vh-19.25rem)] scroll-smooth

overflow-y-hidden hover:overflow-y-scroll
overflow-x-hidden

data-[expanded=false]:w-16
data-[expanded=true]:w-64
";
