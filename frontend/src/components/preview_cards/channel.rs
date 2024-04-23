use invidious::CommonChannel;
use leptos::*;
use leptos_router::NavigateOptions;
use num_format::ToFormattedString;
use phosphor_leptos::{CheckCircle, IconWeight};

use crate::{contexts::RegionConfigCtx, icons::VerifiedIcon};

#[component]
pub fn ChannelPreviewCard(channel: CommonChannel) -> impl IntoView {
	let channel_clone = channel.clone();
	let thumbnail_url = channel_clone.thumbnails.get(3).map(|thumb| thumb.url.clone());
	let id = channel_clone.id;
	let open_channel = move |_| {
		let navigate = leptos_router::use_navigate();
		let id = id.clone();
		request_animation_frame(move || {
			_ = navigate(&format!("/channel?id={}", id), NavigateOptions::default());
		})
	};

	view! {
		<div
			on:click=open_channel
			class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden"
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
	let subscriber_count = channel.subscribers.to_formatted_string(&locale.get().to_num_fmt());
	let verified_check = channel.verified.then_some(
		view! { <CheckCircle weight=IconWeight::Regular class="h-4 w-4 base-content"/> },
	);

	view! {
		<div class="flex flex-col w-full mt-3 space-y-3 px-2 cursor-text">
			<div class="flex flex-row gap-1 item">
				<h1 class="cursor-pointer text-primary font-sans font-semibold text-base line-clamp-2">
					{name}
				</h1>
				{verified_check}
			</div>
			<h2>{subscriber_count}</h2>
		</div>
	}
}

#[component]
pub fn Thumbnail(url: Option<String>) -> impl IntoView {
	let img_loaded = create_rw_signal(false);
	let image_classes = move || match img_loaded.get() {
		true => "w-full aspect-video object-center object-cover bg-neutral rounded-xl".to_string(),
		false => "animate-pulse w-full aspect-video bg-neutral rounded-xl".to_string(),
	};

	view! {
		<div class="w-full max-w-full overflow-hidden rounded-xl">
			<img
				decoding="async"
				on:load=move |_| img_loaded.set(true)
				src=url.clone().map(|url| url.replace("//", "https://"))
				class=image_classes
			/>
		</div>
	}
}
