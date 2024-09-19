use invidious::CommonChannel;
use leptos::*;
use leptos_router::NavigateOptions;
use num_format::ToFormattedString;
use phosphor_leptos::{CheckCircle, IconWeight, UsersThree};

use crate::{contexts::RegionConfigCtx, utils::go_to};

#[component]
pub fn ChannelPreviewCard(channel: CommonChannel) -> impl IntoView {
	let channel_clone = channel.clone();
	let thumbnail_url =
		channel_clone.thumbnails.get(3).map(|thumb| thumb.url.clone());
	let id = channel_clone.id;
	let open_channel = move |_| go_to(format!("/channel?id={id}"));

	view! {
		<div
			on:click=open_channel
			class="flex overflow-hidden flex-col px-4 h-auto"
		>
			<Thumbnail url=thumbnail_url/>
			<Info channel=channel/>
		</div>
	}
}

#[component]
pub fn Info(channel: CommonChannel) -> impl IntoView {
	let locale = expect_context::<RegionConfigCtx>().locale_slice.0;

	let name = channel.name;
	let subscriber_count =
		channel.subscribers.to_formatted_string(&locale.get().to_num_fmt());
	let verified_check = channel.verified.then_some(
		view! { <CheckCircle weight=IconWeight::Regular class="w-4 h-4 base-content"/> },
	);

	view! {
		<div class="flex flex-col px-2 mt-3 space-y-3 w-full cursor-text">
			<div class="flex flex-row gap-1 items-center">
				<h1 class="font-sans text-base font-semibold cursor-pointer text-primary line-clamp-2">
					{name}
				</h1>
				{verified_check}
			</div>
			<div class="flex flex-row gap-1 items-center">
				<UsersThree
					weight=IconWeight::Regular
					class="w-4 h-4 base-content"
				/>
				<h2>{subscriber_count}</h2>
			</div>
		</div>
	}
}

#[component]
pub fn Thumbnail(url: Option<String>) -> impl IntoView {
	let img_loaded = create_rw_signal(false);
	let image_classes = move || {
		if img_loaded.get() {
			"w-full aspect-video object-center object-cover bg-neutral rounded-xl".to_string()
		} else {
			"animate-pulse w-full aspect-video bg-neutral rounded-xl"
				.to_string()
		}
	};

	view! {
		<div class="overflow-hidden w-full max-w-full rounded-xl">
			<img
				decoding="async"
				on:load=move |_| img_loaded.set(true)
				src=url.map(|url| url.replace("//", "https://"))
				class=image_classes
			/>
		</div>
	}
}
