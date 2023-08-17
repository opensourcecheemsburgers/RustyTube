use config::Config;
use invidious::channel::Channel;
use invidious::common::CommonVideo;
use invidious::hidden::CountryCode;
use invidious::universal::{Trending, TrendingCategory, TrendingCategory::*};
use leptos::*;
use num_format::{Locale, ToFormattedString};

#[component]
pub fn VideoPreviewCard(cx: Scope, video: CommonVideo) -> impl IntoView {
	let thumbnail_url = video.thumbnails.first().expect("No thumbnail").url.clone();

	view! {cx,
        <div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden cursor-pointer">
            <VideoPreviewCardThumbnail url=thumbnail_url />
			<VideoPreviewCardInfo video=video />
        </div>
    }
}

#[component]
pub fn VideoPreviewCardInfo(cx: Scope, video: CommonVideo) -> impl IntoView {
	let view_count = video.views.to_formatted_string(&Locale::en);

	view! {cx,
		<div class="flex flex-col w-full mt-3 space-y-3 px-2 cursor-text">
            <h1 class=" font-sans font-semibold text-base line-clamp-2">{&video.title}</h1>
            <div class="flex flex-row flex-wrap font-normal text-sm gap-1">
                <h2 class="cursor-pointer text-info">{&video.author}</h2>
                <p>{"•"}</p>
                <p>{view_count} {r#" views"#}</p>
                <p>{"•"}</p>
                <p>{&video.published_text}</p>
            </div>
        </div>
	}
}

#[component]
pub fn VideoPreviewCardThumbnail(cx: Scope, url: String) -> impl IntoView {
	let (img_classes, set_img_classes) = create_signal(cx, "hidden w-full rounded-xl");
	let (placeholder_classes, set_placeholder_classes) = create_signal(cx, "animate-pulse w-full aspect-w-16 aspect-h-9 bg-base-content rounded-xl");

	let show_image = move |_| {
		// hover:border-2 hover:border-neutral
		set_img_classes.set("w-full rounded-xl hover:scale-110 transition duration-500");
		set_placeholder_classes.set("hidden animate-pulse w-full aspect-w-16 aspect-h-9 bg-base-content rounded-xl")
	};

	view! {cx,
        <div class="w-full max-w-full overflow-hidden rounded-xl">
            <div class=placeholder_classes />
            <img src=url on:load=show_image class=img_classes/>
        </div>
	}
}

#[component]
pub fn VideoPreviewCardPlaceholder(cx: Scope) -> impl IntoView {
	view! {cx,
		<div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden">
			<div class="animate-pulse w-full aspect-w-16 aspect-h-9 bg-base-content rounded-xl" />
			<div class="flex flex-col w-full mt-3 space-y-3 px-2">
				<div class="animate-pulse w-full h-2 rounded-xl bg-base-content"></div>
				<div class="animate-pulse w-full h-2 rounded-xl bg-base-content"></div>
				<div class="animate-pulse w-[35%] h-2 rounded-xl bg-base-content"></div>
			</div>
		</div>
	}
}

#[component]
pub fn VideoPreviewCardPlaceholderArray(cx: Scope) -> impl IntoView {
	view! {cx,
        <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-1rem-128px)] overflow-y-scroll">
            {
                let mut i = 0;
                let mut vec = vec![];

                while i < 50 {
                    vec.push(view!
                        {cx,
                        <div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden">
                            <div class="animate-pulse w-full aspect-w-16 aspect-h-9 bg-base-content rounded-xl" />
                            <div class="flex flex-col w-full mt-3 space-y-3 px-2">
                                <div class="animate-pulse w-full h-2 rounded-xl bg-base-content"></div>
                                <div class="animate-pulse w-full h-2 rounded-xl bg-base-content"></div>
                                <div class="animate-pulse w-[35%] h-2 rounded-xl bg-base-content"></div>
                            </div>
                        </div>
                        }
                );
                    i = i + 1;
                }
                vec.collect_view(cx)
            }
        </div>
    }
}