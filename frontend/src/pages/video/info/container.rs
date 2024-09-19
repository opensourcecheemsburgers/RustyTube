use invidious::{Dislikes, Formats, Video};
use leptos::*;
use leptos_router::create_query_signal;
use num_format::ToFormattedString;
use phosphor_leptos::{
	CalendarBlank, DownloadSimple, Eye, IconWeight, ShareNetwork, ThumbsDown,
	ThumbsUp,
};

use crate::{
	components::{ChannelRoll, FerrisError},
	contexts::{PlayerState, RegionConfigCtx},
	resources::{SubscriptionsCtx, VideoResource},
};

#[component]
pub fn VideoInfo() -> impl IntoView {
	view! {
		<Suspense fallback=move || {
			().into_view()
		}>
			{move || {
				expect_context::<VideoResource>()
					.resource
					.get()
					.map(|res| match res {
						Ok(video) => view! { <VideoInfoContent video=video/> },
						Err(err) => view! { <FerrisError error=err/> },
					})
			}}

		</Suspense>
	}
}

#[component]
pub fn VideoInfoContent(video: Video) -> impl IntoView {
	let locale = expect_context::<RegionConfigCtx>().locale_slice.0;

	let title = video.title;
	let published = video.published_text;
	let views =
		move || video.views.to_formatted_string(&locale.get().to_num_fmt());
	let likes =
		move || video.likes.to_formatted_string(&locale.get().to_num_fmt());
	let author = video.author;
	let author_id = video.author_id.clone();
	let sub_count_text = video.sub_count_text.clone();
	let author_thumb_url =
		video.author_thumbnails.first().cloned().map(|thumb| thumb.url);
	let description = video.description_html;

	let formats = Formats::from((
		video.adaptive_formats.clone(),
		video.format_streams.clone(),
	));

	let img_loaded = create_rw_signal(false);
	let image_classes = move || {
		if img_loaded.get() {
			"h-16 w-16 rounded-full".to_string()
		} else {
			"h-16 w-16 animate-pulse rounded-full bg-neutral".to_string()
		}
	};

	let dislikes = Resource::local(
		move || video.id.clone(),
		|id| async move { Dislikes::fetch_dislikes(&id).await },
	);

	let dislikes_view = move || {
		dislikes.get().map(|dislikes| {
			view! {
				<div class="flex flex-row gap-1 items-center">
					<ThumbsDown
						weight=IconWeight::Regular
						class="w-4 h-4 base-content"
					/>
					<p>
						{dislikes
							.unwrap_or_default()
							.to_formatted_string(&locale.get().to_num_fmt())}
					</p>
				</div>
			}
			.into_view()
		})
	};

	view! {
		<div class="flex flex-row justify-between p-4 w-full rounded-lg h-max bg-base-200">
			<div class="flex flex-col w-full">
				<h1 class="text-lg font-semibold md:text-xl">
					{title.clone()}
				</h1>

				<div class="p-0 m-0 text-xs md:text-sm collapse collapse-arrow w-fit h-fit">
					<input type="checkbox"/>
					<div class="flex flex-row flex-wrap gap-2 items-center pl-0 collapse-title w-fit">
						<div class="flex flex-row gap-1 items-center">
							<Eye
								weight=IconWeight::Regular
								class="w-4 h-4 base-content"
							/>
							<p>{views}</p>
						</div>
						<p>{"•"}</p>
						<div class="flex flex-row gap-2 items-center">
							<div class="flex flex-row gap-1 items-center">
								<ThumbsUp
									weight=IconWeight::Regular
									class="w-4 h-4 base-content"
								/>
								<p>{likes}</p>
							</div>
							{dislikes_view}
						</div>
						<p>{"•"}</p>
						<div class="flex flex-row gap-1 items-center">
							<CalendarBlank
								weight=IconWeight::Regular
								class="w-4 h-4 base-content"
							/>
							<p>{published}</p>
						</div>
					</div>

					<div class="pl-0 collapse-content">
						<div
							class="flex flex-col gap-y-4 [&_a]:link [&_a]:link-info [&_a]:no-underline"
							inner_html=description
						></div>
					</div>
				</div>

				<div class="flex flex-row flex-wrap gap-y-4 justify-between items-center mt-2 w-full sm:flex-nowrap">
					<ChannelRoll
						channel=author
						channel_id=author_id
						sub_count=sub_count_text
						image_url=author_thumb_url.unwrap_or_default()
					/>
					<div class="flex flex-row gap-x-2 justify-center items-end">
						<DownloadsDropdown formats=formats title=title/>
						<ShareDropdown/>
					</div>
				</div>
			</div>
		</div>
	}
}

#[component]
pub fn DownloadsDropdown(formats: Formats, title: String) -> impl IntoView {
	view! {
		<div class="z-20 dropdown dropdown-bottom sm:dropdown-end">
			<div
				tabindex="0"
				role="button"
				class="btn btn-circle btn-accent btn-outline"
			>
				<DownloadSimple
					weight=IconWeight::Regular
					class="w-6 h-6 base-content"
				/>
			</div>
			<ul
				tabindex="0"
				class="py-3 px-1.5 mt-2 w-max rounded-xl menu dropdown-content shadow-dropdown bg-base-200 h-max"
			>
				<DownloadsDropdownList formats=formats title=title/>
			</ul>
		</div>
	}
}

#[component]
pub fn DownloadsDropdownList(formats: Formats, title: String) -> impl IntoView {
	let audio_formats_view = {
		formats
			.audio_formats
			.into_iter()
			.map(|format| {
				let quality_str = format.audio_quality.to_string();

				let title = title.clone();
				view! {
					<a
						href=format.url
						target="_blank"
						class="lowercase btn btn-xs btn-ghost md:btn-sm"
						download=title
					>
						{quality_str}
					</a>
				}
			})
			.collect_view()
	};

	let adaptive_formats_view = {
		formats
			.video_formats
			.into_iter()
			.map(|format| {
				let info_str = format.clone().container.map_or(
					format.quality_label.to_string(),
					|container| {
						format!("{} - ({})", format.quality_label, container)
					},
				);

				let title = title.clone();
				view! {
					<a
						href=format.url
						target="_blank"
						class="lowercase btn btn-xs btn-ghost md:btn-sm"
						download=title
					>
						{info_str}
					</a>
				}
			})
			.collect_view()
	};

	let legacy_formats_view = {
		formats
			.legacy_formats
			.into_iter()
			.map(|format| {
				let quality_str = format.quality_label.to_string();

				let title = title.clone();
				view! {
					<a
						href=format.url
						target="_blank"
						class="lowercase btn btn-xs btn-ghost md:btn-sm"
						download=title
					>
						{quality_str}
					</a>
				}
			})
			.collect_view()
	};

	view! {
		<div class="flex flex-row gap-x-4 p-2 w-max rounded-lg h-max bg-base-200">
			<div class="flex flex-col items-center">
				<h1>Audio</h1>
				<div class="flex overflow-y-scroll flex-col my-4 h-64">
					{audio_formats_view}
				</div>
			</div>

			<div class="flex flex-col items-center">
				<h1>Legacy</h1>
				<div class="flex overflow-y-scroll flex-col my-4 h-64">
					{legacy_formats_view}
				</div>
			</div>

			<div class="flex flex-col items-center">
				<h1>Dash</h1>
				<div class="flex overflow-y-scroll flex-col my-4 h-64">
					{adaptive_formats_view}
				</div>
			</div>
		</div>
	}
}

#[derive(Clone, Copy)]
pub enum LinkType {
	RustyTube,
	YouTube,
}

#[component]
pub fn ShareDropdown() -> impl IntoView {
	let include_timestamp = create_rw_signal(false);
	let timestamp_input = create_node_ref();
	let toggle_timestamp = move |_| {
		let timestamp = timestamp_input.get().map_or(
			false,
			|timestamp_input: HtmlElement<html::Input>| {
				timestamp_input.checked()
			},
		);
		include_timestamp.set(timestamp);
	};

	let link_type = create_rw_signal(LinkType::RustyTube);
	let set_rt_link_type = move |_| link_type.set(LinkType::RustyTube);
	let set_yt_link_type = move |_| link_type.set(LinkType::YouTube);
	let current_time = expect_context::<PlayerState>().current_time;
	let link_text = move || {
		let video_id =
			create_query_signal::<String>("id").0.get().unwrap_or_default();
		if matches!(link_type.get(), LinkType::RustyTube) {
			if include_timestamp.get() {
				format!(
					"https://rustytube.rs/player?id={}&time={}",
					video_id,
					current_time.get()
				)
			} else {
				format!("https://rustytube.rs/player?id={video_id}")
			}
		} else if include_timestamp.get() {
			format!(
				"https://youtube.com/watch?v={}&t={}s",
				video_id,
				current_time.get()
			)
		} else {
			format!("https://youtube.com/watch?v={video_id}")
		}
	};

	view! {
		<div class="z-20 dropdown dropdown-bottom sm:dropdown-end">
			<div
				tabindex="0"
				role="button"
				class="btn btn-circle btn-outline btn-accent"
			>
				<ShareNetwork
					weight=IconWeight::Regular
					class="w-6 h-6 base-content"
				/>
			</div>

			<div
				tabindex="0"
				class="p-4 mt-2 space-y-4 w-max rounded-lg dropdown-content h-max bg-base-200 shadow-dropdown"
			>
				<div tabindex="0" class="flex flex-row gap-2">
					<button
						on:click=set_rt_link_type
						class="btn btn-outline btn-accent btn-xs md:btn-sm"
					>
						RustyTube Link
					</button>
					<button
						on:click=set_yt_link_type
						class="btn btn-outline btn-accent btn-xs md:btn-sm"
					>
						YouTube Link
					</button>
				</div>

				<div
					tabindex="0"
					class="flex flex-row gap-x-1 items-center py-1 px-3 w-full rounded-lg h-max btn-accent bg-accent"
				>
					<p class="font-mono md:text-xs text-[8px] text-accent-content">
						{link_text}
					</p>
				</div>

				<div tabindex="0" class="form-control">
					<label class="cursor-pointer label">
						<span class="label-text">Include Timestamp</span>
						<input
							on:change=toggle_timestamp
							_ref=timestamp_input
							type="checkbox"
							checked=include_timestamp
							class="checkbox checkbox-accent"
						/>
					</label>
				</div>

			</div>
		</div>
	}
}

#[component]
pub fn SubscribeBtn(
	author: String,
	author_id: String,
	sub_count_text: String,
) -> impl IntoView {
	let subscriptions_ctx = expect_context::<SubscriptionsCtx>();

	let author = StoredValue::new(author);
	let author_id = StoredValue::new(author_id);

	let add_sub = create_action(|args: &(String, String, SubscriptionsCtx)| {
		let name = args.0.clone();
		let id = args.1.clone();
		let subscriptions_ctx = args.2;
		async move { subscriptions_ctx.add_subscription(&id, &name).await }
	});

	let remove_sub = create_action(|args: &(String, SubscriptionsCtx)| {
		let id = args.0.clone();
		let subscriptions_ctx = args.1;
		async move { subscriptions_ctx.remove_subscription(&id).await }
	});

	let is_subscribed = move || {
		subscriptions_ctx
			.0
			.get()
			.channels
			.into_iter()
			.any(|sub| sub.id.eq_ignore_ascii_case(&author_id.get_value()))
	};

	let btn_text = move || {
		if is_subscribed() {
			"Subscribed"
		} else {
			"Subscribe"
		}
	};

	let is_subscribed = move || {
		subscriptions_ctx
			.0
			.get()
			.channels
			.into_iter()
			.any(|sub| sub.id.eq_ignore_ascii_case(&author_id.get_value()))
	};

	let on_click = move |_| {
		if is_subscribed() {
			remove_sub.dispatch((author_id.get_value(), subscriptions_ctx));
		} else {
			add_sub.dispatch((
				author.get_value(),
				author_id.get_value(),
				subscriptions_ctx,
			));
		}
	};

	view! {
		<button on:click=on_click class="w-32 btn btn-primary btn-xs">
			<div class="flex flex-row gap-3 justify-between">
				<p>{btn_text}</p>
				<p>{sub_count_text}</p>
			</div>
		</button>
	}
}

#[component]
pub fn VideoInfoPlaceholder() -> impl IntoView {
	view! {
		<div class="flex flex-col space-y-4 w-full rounded-lg bg-base-200">
			<h1>{}</h1>
		</div>
	}
}
