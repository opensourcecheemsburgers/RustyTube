use invidious::CommonVideo;
use leptos::*;
use num_format::{Locale, ToFormattedString};
use rustytube_error::RustyTubeError;

use crate::icons::FerrisWtfIcon;

#[component]
pub fn VideoPreviewCard(
	cx: Scope,
	video_id: String,
	thumbnail_url: String,
	title: String,
	author: String,
	views: u64,
	published: String
) -> impl IntoView {
	view! {cx,
        <div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden">
            <VideoPreviewCardThumbnail url=thumbnail_url video_id=video_id />
			<VideoPreviewCardInfo title=title author=author views=views published=published />
        </div>
    }
}

#[component]
pub fn VideoPreviewCardInfo(
	cx: Scope,
	title: String,
	author: String,
	views: u64,
	published: String
) -> impl IntoView {
	let view_count = views.to_formatted_string(&Locale::en);

	view! {cx,
		<div class="flex flex-col w-full mt-3 space-y-3 px-2 cursor-text">
            <h1 class=" font-sans font-semibold text-base line-clamp-2">{title}</h1>
            <div class="flex flex-row flex-wrap font-normal text-sm gap-1">
                <h2 class="cursor-pointer text-primary">{author}</h2>
                <p>{"•"}</p>
                <p>{view_count} {r#" views"#}</p>
                <p>{"•"}</p>
                <p>{published}</p>
            </div>
        </div>
	}
}

#[component]
pub fn VideoPreviewCardThumbnail(cx: Scope, url: String, video_id: String) -> impl IntoView {
	let open_video = move |_| {
		let navigate = leptos_router::use_navigate(cx);
		let id = video_id.clone();
		request_animation_frame(move || {
			_ = navigate(&format!("/player?id={}", id.clone()), Default::default());
		})
	};

    let img_loaded = create_rw_signal(cx, false);
	let image_classes = move || match img_loaded.get() {
		true => "w-full aspect-video object-center object-cover bg-base-content rounded-xl".to_string(),
		false => "animate-pulse w-full aspect-video bg-base-content rounded-xl".to_string()
	};
    
	view! {cx,
        <div on:click=open_video class="w-full max-w-full overflow-hidden rounded-xl">
            <img 
                decoding="sync"
                on:load=move |_| img_loaded.set(true)
                src=url 
                class={image_classes}
            />
        </div>
	}
}


#[component]
pub fn VideoPreviewCardPlaceholderArray(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-13rem)] overflow-y-scroll">
            {(0..50).map(|_| view! {cx, <VideoPreviewCardPlaceholder />}).collect_view(cx)}
        </div>
    }
}

#[component]
pub fn VideoPreviewCardPlaceholder(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden">
            <div class="animate-pulse w-full aspect-video bg-base-content rounded-xl" />
            <div class="flex flex-col w-full mt-3 space-y-3 px-2">
                <div class="animate-pulse w-full h-2 rounded-xl bg-base-content"></div>
                <div class="animate-pulse w-full h-2 rounded-xl bg-base-content"></div>
                <div class="animate-pulse w-[35%] h-2 rounded-xl bg-base-content"></div>
            </div>
        </div>
    }
}