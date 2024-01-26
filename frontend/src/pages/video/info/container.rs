use invidious::{Dislikes, Formats, Video};
use leptos::*;
use num_format::{Locale, ToFormattedString};

use crate::{
	components::FerrisError,
	contexts::{PlayerState, SubscriptionsCtx},
	icons::{CalendarIcon, DislikeIcon, DownloadIcon, LikeIcon, ShareIcon, ViewsIcon},
	pages::video::page::VideoResource,
	utils::get_current_video_query_signal,
};

#[component]
pub fn VideoInfo(video_resource: VideoResource) -> impl IntoView {
	view! {
		<Suspense fallback=move || {
			().into_view()
		}>
			{move || {
				video_resource
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
	let title = video.title;
	let published = video.published_text;
	let views = video.views.to_formatted_string(&Locale::en);
	let likes = video.likes.to_formatted_string(&Locale::en);
	let author = video.author;
	let author_id = video.author_id.clone();
	let sub_count_text = video.sub_count_text;
	let author_thumb_url = video.author_thumbnails.first().cloned().map(|thumb| thumb.url);
	let description = video.description_html;

	let formats = Formats::from((video.adaptive_formats.clone(), video.format_streams.clone()));

	let img_loaded = create_rw_signal(false);
	let image_classes = move || match img_loaded.get() {
		true => "h-16 w-16 rounded-full".to_string(),
		false => "h-16 w-16 animate-pulse rounded-full bg-neutral".to_string(),
	};

	let dislikes = create_resource(
		move || video.id.clone(),
		|id| async move { Dislikes::fetch_dislikes(&id).await },
	);

	let dislikes_view = move || {
		dislikes.get().map(|dislikes| {
			view! {
				<div class="flex flex-row items-center gap-1">
					<DislikeIcon/>
					<p>{dislikes.unwrap_or_default()}</p>
				</div>
			}
			.into_view()
		})
	};

	let channel_id = video.author_id.clone();
	let go_to_channel_page = move |_| {
		let navigate = leptos_router::use_navigate();
		let channel_id = channel_id.clone();
		request_animation_frame(move || {
			_ = navigate(&format!("/channel?id={}", channel_id), Default::default());
		})
	};

	view! {
		<div class="flex h-max w-full flex-row justify-between rounded-lg bg-base-200 p-4">
			<div class="flex w-full flex-col">
				<h1 class="text-xl font-semibold">{title.clone()}</h1>

				<div class="p-0 m-0 collapse collapse-arrow w-fit h-fit">
					<input type="checkbox"/>
					<div class="collapse-title pl-0 flex flex-row flex-wrap items-center gap-2 w-fit">
						<div class="flex flex-row items-center gap-1">
							<ViewsIcon/>
							<p>{views}</p>
						</div>
						<p>{"•"}</p>
						<div class="flex flex-row items-center gap-2">
							<div class="flex flex-row items-center gap-1">
								<LikeIcon/>
								<p>{likes}</p>
							</div>
							{dislikes_view}
						</div>
						<p>{"•"}</p>
						<div class="flex flex-row items-center gap-1">
							<CalendarIcon/>
							<p>{published}</p>
						</div>
					</div>

					<div class="collapse-content pl-0">
						<div
							class="flex flex-col gap-y-4 [&_a]:link [&_a]:link-info [&_a]:no-underline"
							inner_html=description
						></div>
					</div>
				</div>

				<div class="mt-2 flex w-full flex-row items-center justify-between space-x-4">
					<div class="flex flex-row space-x-4">
						<img
							on:click=go_to_channel_page
							on:load=move |_| img_loaded.set(true)
							src=author_thumb_url
							class=image_classes
						/>
						<div class="flex flex-col space-y-2">
							<p class="text-xl font-semibold">{author.clone()}</p>
							<SubscribeBtn
								author=author
								author_id=author_id
								sub_count_text=sub_count_text
							/>
						</div>
					</div>
					<div class="flex flex-row items-end justify-center space-x-2">
						<DownloadsDropdown formats=formats title=title.clone()/>
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
		<div class="dropdown dropdown-bottom dropdown-end z-20">
			<div tabindex="0" role="button" class="btn btn-circle btn-accent btn-outline">
				<DownloadIcon/>
			</div>
			<ul
				tabindex="0"
				class="menu dropdown-content mt-2 px-1.5 py-3 shadow-dropdown bg-base-200 rounded-xl w-max h-max"
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
				let quality_str = format.audio_quality.clone().to_string();

				let title = title.clone();
				view! {
					<a
						href=format.url
						target="_blank"
						class="btn btn-sm lowercase btn-ghost"
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
						format!(
							"{} - ({})",
							format.quality_label.to_string(),
							container.to_string()
						)
					},
				);

				let title = title.clone();
				view! {
					<a
						href=format.url
						target="_blank"
						class="btn btn-sm lowercase btn-ghost"
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
				let quality_str = format.quality_label.clone().to_string();

				let title = title.clone();
				view! {
					<a
						href=format.url
						target="_blank"
						class="btn btn-sm lowercase btn-ghost"
						download=title
					>
						{quality_str}
					</a>
				}
			})
			.collect_view()
	};

	view! {
		<div class="flex h-max w-max flex-row space-x-4 rounded-lg bg-base-200 p-2">
			<div class="flex flex-col items-center">
				<h1>Audio</h1>
				<div class="my-4 flex flex-col h-64 overflow-y-scroll">{audio_formats_view}</div>
			</div>

			<div class="flex flex-col items-center">
				<h1>Legacy</h1>
				<div class="my-4 flex flex-col h-64 overflow-y-scroll">{legacy_formats_view}</div>
			</div>

			<div class="flex flex-col items-center">
				<h1>Dash</h1>
				<div class="my-4 flex flex-col h-64 overflow-y-scroll">{adaptive_formats_view}</div>
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
		let timestamp = timestamp_input
			.get()
			.map_or(false, |timestamp_input: HtmlElement<html::Input>| timestamp_input.checked());
		include_timestamp.set(timestamp);
	};

	let video_id = get_current_video_query_signal().0.get().unwrap_or_default();

	let link_type = create_rw_signal(LinkType::RustyTube);
	let set_rt_link_type = move |_| link_type.set(LinkType::RustyTube);
	let set_yt_link_type = move |_| link_type.set(LinkType::YouTube);
	let current_time = expect_context::<PlayerState>().current_time;
	let link_text = move || match link_type.get() {
		LinkType::RustyTube => match include_timestamp.get() {
			true => {
				format!("https://rustytube.rs/player?id={}&time={}", video_id, current_time.get())
			}
			false => format!("https://rustytube.rs/player?id={}", video_id),
		},
		LinkType::YouTube => match include_timestamp.get() {
			true => format!("https://youtube.com/watch?v={}&t={}s", video_id, current_time.get()),
			false => format!("https://youtube.com/watch?v={}", video_id),
		},
	};

	view! {
		<div class="dropdown dropdown-bottom dropdown-end z-20">
			<div tabindex="0" role="button" class="btn btn-circle btn-outline btn-accent">
				<ShareIcon/>
			</div>

			<div
				tabindex="0"
				class="dropdown-content h-max w-max mt-2 space-y-4 rounded-lg bg-base-200 p-4 shadow-dropdown"
			>
				<div tabindex="0" class="flex flex-row gap-2">
					<button on:click=set_rt_link_type class="btn btn-outline btn-accent btn-sm">
						RustyTube Link
					</button>
					<button on:click=set_yt_link_type class="btn btn-outline btn-accent btn-sm">
						YouTube Link
					</button>
				</div>

				<div
					tabindex="0"
					class="flex h-max w-full flex-row items-center space-x-1 rounded-lg btn-accent px-3 py-1 bg-accent"
				>
					<p class="font-mono text-xs text-accent-content">{link_text}</p>
				</div>

				<div tabindex="0" class="form-control">
					<label class="label cursor-pointer">
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
pub fn SubscribeBtn(author: String, author_id: String, sub_count_text: String) -> impl IntoView {
	let subscriptions_ctx = expect_context::<SubscriptionsCtx>();

	let author = StoredValue::new(author);
	let author_id = StoredValue::new(author_id);

	let add_sub = create_action(|args: &(String, String, SubscriptionsCtx)| {
		let name = args.0.clone();
		let id = args.1.clone();
		let subscriptions_ctx = args.2.clone();
		async move { subscriptions_ctx.add_subscription(&id, &name).await }
	});

	let remove_sub = create_action(|args: &(String, SubscriptionsCtx)| {
		let id = args.0.clone();
		let subscriptions_ctx = args.1.clone();
		async move { subscriptions_ctx.remove_subscription(&id).await }
	});

	let is_subscribed = move || {
		subscriptions_ctx
			.0
			.get()
			.channels
			.into_iter()
			.find(|sub| sub.id.eq_ignore_ascii_case(&author_id.get_value()))
			.is_some()
	};

	let btn_text = move || match is_subscribed() {
		true => "Subscribed",
		false => "Subscribe",
	};

	let is_subscribed = move || {
		subscriptions_ctx
			.0
			.get()
			.channels
			.into_iter()
			.find(|sub| sub.id.eq_ignore_ascii_case(&author_id.get_value()))
			.is_some()
	};

	let on_click = move |_| match is_subscribed() {
		true => remove_sub.dispatch((author_id.get_value(), subscriptions_ctx)),
		false => add_sub.dispatch((author.get_value(), author_id.get_value(), subscriptions_ctx)),
	};

	view! {
		<button on:click=on_click class="btn btn-primary btn-xs w-32">
			<div class="flex flex-row justify-between gap-3">
				<p>{btn_text}</p>
				<p>{sub_count_text}</p>
			</div>
		</button>
	}
}

#[component]
pub fn VideoInfoPlaceholder() -> impl IntoView {
	view! {
		<div class="flex flex-col space-y-4 w-full bg-base-200 rounded-lg">
			<h1>{}</h1>
		</div>
	}
}
