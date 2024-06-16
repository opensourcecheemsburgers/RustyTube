use invidious::{ChannelThumb, Subscription, Subscriptions};
use leptos::*;
use phosphor_leptos::{
	FireSimple, GearSix, Heart, IconWeight, Queue, RssSimple, TrendUp,
};
use rustytube_error::RustyTubeError;
use utils::get_element_by_id;
use web_sys::HtmlDialogElement;

use crate::{
	components::{
		donate_modal::{DonateModal, DONATE_MODAL_ID},
		FerrisError,
	},
	icons::FerrisIcon,
	resources::SubscriptionsThumbnailsResource,
	utils::{go_to, i18n},
};

#[derive(Clone, Copy)]
pub struct ExpandedCtx(pub RwSignal<String>);

#[component]
pub fn Sidebar() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	view! {
		<div data-expanded=expanded class=SIDEBAR_CLASSES>
			<SidebarHeader/>
			<div>
				<div class="border-b-[1px] border-b-primary">
					<SubscriptionsButton/>
					<TrendingButton/>
					<PopularButton/>
				// <PlaylistsButton/>
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
pub fn SidebarHeader() -> impl IntoView {
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
pub fn SubscriptionsButton() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	view! {
		<div
			data-expanded=expanded
			data-tip=i18n("sidebar.subscriptions")
			class=SIDEBAR_TOOLTIP_CLASSES
		>
			<button
				on:click=|_| go_to("/subscriptions")
				data-expanded=expanded
				class=SIDEBAR_ITEM_CLASSES
			>
				<RssSimple
					weight=IconWeight::Regular
					class="base-content"
					size="24px"
				/>
				<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
					{i18n("sidebar.subscriptions")}
				</p>
			</button>
		</div>
	}
}

#[component]
pub fn TrendingButton() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	view! {
		<div
			data-expanded=expanded
			data-tip=i18n("sidebar.trending")
			class=SIDEBAR_TOOLTIP_CLASSES
		>
			<button
				on:click=move |_| go_to("/trending")
				data-expanded=expanded
				class=SIDEBAR_ITEM_CLASSES
			>
				<TrendUp
					weight=IconWeight::Regular
					class="base-content"
					size="24px"
				/>
				<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
					{i18n("sidebar.trending")}
				</p>
			</button>
		</div>
	}
}

#[component]
pub fn PopularButton() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	view! {
		<div
			data-expanded=expanded
			data-tip=i18n("sidebar.popular")
			class=SIDEBAR_TOOLTIP_CLASSES
		>
			<button
				on:click=move |_| go_to("/popular")
				data-expanded=expanded
				class=SIDEBAR_ITEM_CLASSES
			>
				<FireSimple
					weight=IconWeight::Regular
					class="base-content"
					size="24px"
				/>
				<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
					{i18n("sidebar.popular")}
				</p>
			</button>
		</div>
	}
}

#[component]
pub fn PlaylistsButton() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	view! {
		<div
			data-expanded=expanded
			data-tip=i18n("sidebar.playlists")
			class=SIDEBAR_TOOLTIP_CLASSES
		>
			<button
				on:click=|_| go_to("/playlists")
				data-expanded=expanded
				class=SIDEBAR_ITEM_CLASSES
			>
				<Queue
					weight=IconWeight::Regular
					class="base-content"
					size="24px"
				/>
				<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
					{i18n("sidebar.playlists")}
				</p>
			</button>
		</div>
	}
}

#[component]
pub fn Subs() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;
	let channel_thumbs_ctx =
		expect_context::<SubscriptionsThumbnailsResource>().resource;

	view! {
		<div data-expanded=expanded class=SIDEBAR_SUBS_CLASSES>
			{move || {
				channel_thumbs_ctx
					.get()
					.map(|channel_thumbs| match channel_thumbs {
						Ok(channel_thumbs) => {
							view! { <SubsInner results=channel_thumbs/> }
						}
						Err(err) => view! { <FerrisError error=err/> },
					})
			}}

		</div>
	}
}

#[component]
pub fn SubsInner(
	results: Vec<Result<ChannelThumb, RustyTubeError>>,
) -> impl IntoView {
	let mut channels = results
		.into_iter()
		.filter_map(std::result::Result::ok)
		.collect::<Vec<ChannelThumb>>();
	channels.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

	channels
		.into_iter()
		.map(|channel| view! { <ChannelButton channel=channel/> })
		.collect_view()
}

#[component]
pub fn ChannelButton(channel: ChannelThumb) -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;
	let thumb_url = channel.thumbnails.first().map(|thumb| thumb.url.clone());

	let go_to_channel_page = move |_| {
		let author_id = channel.id.clone();
		go_to(format!("/channel?id={author_id}"));
	};

	view! {
		<button
			on:click=go_to_channel_page
			data-expanded=expanded
			class=SIDEBAR_ITEM_CLASSES
		>
			<img src=thumb_url class="w-6 h-6 rounded-full"/>
			<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
				{channel.name}
			</p>
		</button>
	}
}

#[component]
pub fn ChannelButtonPlaceholder(sub: Subscription) -> impl IntoView {
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
pub fn SettingsButton() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	view! {
		<div
			on:click=|_| go_to("/settings")
			data-expanded=expanded
			data-tip=i18n("sidebar.settings")
			class=SIDEBAR_TOOLTIP_CLASSES
		>
			<button data-expanded=expanded class=SIDEBAR_ITEM_CLASSES>
				<GearSix
					weight=IconWeight::Regular
					class="base-content"
					size="24px"
				/>
				<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
					{i18n("sidebar.settings")}
				</p>
			</button>
		</div>
	}
}

#[component]
pub fn DonateButton() -> impl IntoView {
	let expanded = expect_context::<ExpandedCtx>().0;

	let open_donate_modal = move |_| {
		let modal = get_element_by_id::<HtmlDialogElement>(DONATE_MODAL_ID);
		if let Ok(modal) = modal {
			modal.set_open(true);
		}
	};

	view! {
		<div
			data-expanded=expanded
			data-tip=i18n("sidebar.donate")
			class=SIDEBAR_TOOLTIP_CLASSES
		>
			<button
				on:click=open_donate_modal
				data-expanded=expanded
				class=SIDEBAR_ITEM_CLASSES
			>
				<Heart
					weight=IconWeight::Regular
					class="base-content"
					size="24px"
				/>
				<p data-expanded=expanded class=SIDEBAR_ITEM_TEXT_CLASSES>
					{i18n("sidebar.donate")}
				</p>
			</button>
		</div>
	}
}

pub const SIDEBAR_CLASSES: &str = "
hidden lg:landscape:!flex flex-col min-h-screen max-h-screen bg-base-200 transition-all \
                                           duration-300

data-[expanded=false]:w-16
data-[expanded=true]:w-64
";

pub const SIDEBAR_HEADER_CLASSES: &str = "
btn btn-ghost h-16 transition-all duration-300 flex flex-row flex-nowrap gap-x-0 overflow-hidden

data-[expanded=false]:w-16
data-[expanded=true]:w-64

data-[expanded=false]:items-center
data-[expanded=false]:justify-center
data-[expanded=true]:items-center
data-[expanded=true]:justify-start
";

pub const SIDEBAR_HEADER_TEXT_CLASSES: &str = "
normal-case font-display font-medium text-2xl

data-[expanded=false]:hidden
data-[expanded=false]:opacity-0
data-[expanded=false]:absolute

data-[expanded=false]:static
data-[expanded=true]:opacity-100
data-[expanded=true]:transition-opacity
data-[expanded=true]:duration-1000
";

pub const SIDEBAR_TOOLTIP_CLASSES: &str = "
data-[expanded=false]:tooltip
data-[expanded=false]:tooltip-primary
data-[expanded=false]:tooltip-right
";

pub const SIDEBAR_ITEM_CLASSES: &str = "
btn btn-ghost transition-all duration-300 flex flex-row flex-nowrap gap-x-2

data-[expanded=false]:w-16
data-[expanded=true]:w-64

data-[expanded=false]:items-center
data-[expanded=false]:justify-center

data-[expanded=true]:items-center
data-[expanded=true]:justify-start
";

pub const SIDEBAR_ITEM_TEXT_CLASSES: &str = "
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
pub const SIDEBAR_SUBS_CLASSES: &str = "
h-[calc(100svh-19.25rem)] min-h-[calc(100svh-19.25rem)] max-h-[calc(100svh-19.25rem)] scroll-smooth

overflow-y-hidden hover:overflow-y-scroll
overflow-x-hidden

data-[expanded=false]:w-16
data-[expanded=true]:w-64
";
