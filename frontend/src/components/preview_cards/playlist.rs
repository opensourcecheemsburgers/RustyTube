use invidious::CommonPlaylist;
use leptos::*;
use leptos_router::NavigateOptions;
use num_format::{Locale, ToFormattedString};

use crate::icons::VerifiedIcon;

#[component]
pub fn PlaylistPreviewCard(playlist: CommonPlaylist) -> impl IntoView {
    let playlist_clone = playlist.clone();
    let thumbnail_url = playlist_clone.thumbnail;
    let id = playlist_clone.id;
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
            <Info playlist=playlist/>
        </div>
    }
}

#[component]
pub fn Info(playlist: CommonPlaylist) -> impl IntoView {
    let name = playlist.title;
    let author = playlist.author;
    let video_count = playlist.video_count.to_formatted_string(&Locale::en);
    let verified_check = playlist
        .author_verified
        .then_some(view! { <VerifiedIcon/> });

    view! {
        <div class="flex flex-col w-full mt-3 space-y-3 px-2 cursor-text">
            <h1 class="font-sans font-semibold text-base line-clamp-2">{name}</h1>
            <div class="flex flex-row flex-wrap items-center font-normal text-sm gap-1">
                <h2 class="cursor-pointer text-primary">{author}</h2>
                {verified_check}
                <p>{"•"}</p>
                <p>{video_count} {r#" videos"#}</p>
            </div>
        </div>
    }
}

#[component]
pub fn Thumbnail(url: String) -> impl IntoView {
    let img_loaded = create_rw_signal(false);
    let image_classes = move || match img_loaded.get() {
        true => {
            "w-full aspect-video object-center object-cover bg-base-content rounded-xl".to_string()
        }
        false => "animate-pulse w-full aspect-video bg-base-content rounded-xl".to_string(),
    };

    view! {
        <div class="w-full max-w-full overflow-hidden rounded-xl">
            <img
                decoding="async"
                on:load=move |_| img_loaded.set(true)
                src=url
                class=image_classes
            />
        </div>
    }
}

