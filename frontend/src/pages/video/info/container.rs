use crate::components::FerrisError;
use crate::icons::{CalendarIcon, DownloadIcon, LikeIcon, PlaylistAddIcon, ShareIcon, ViewsIcon};
use crate::pages::video::page::VideoResource;
use invidious::Video;
use leptos::*;
use num_format::{Locale, ToFormattedString};

#[component]
pub fn VideoInfo(video_resource: VideoResource) -> impl IntoView {
    let video_info_view = move || {
        video_resource.read().map(|res| match res {
            Ok(video) => view! { <VideoInfoContent video=video/> },
            Err(err) => view! { <FerrisError error=err/> },
        })
    };

    view! { <Suspense fallback=move || view! { <VideoInfoPlaceholder/> }>{video_info_view}</Suspense> }
}

#[component]
pub fn VideoInfoContent(video: Video) -> impl IntoView {
    let title = video.title;
    let published = video.published_text;
    let views = video.views.to_formatted_string(&Locale::en);
    let likes = video.likes.to_formatted_string(&Locale::en);
    let author = video.author;
    let author_url = video.author_url;
    let author_thumb_url = video
        .author_thumbnails
        .first()
        .cloned()
        .map(|thumb| thumb.url);

    let img_loaded = create_rw_signal(false);
    let image_classes = move || match img_loaded.get() {
        true => "h-16 w-16 rounded-full".to_string(),
        false => "h-16 w-16 animate-pulse rounded-full bg-neutral".to_string(),
    };

    view! {
        <div class="flex h-max w-full flex-row justify-between rounded-lg bg-base-200 p-4">
            <div class="flex w-full flex-col">
                <h1 class="text-xl font-semibold">{title}</h1>
                <div class="mt-2 flex flex-row flex-wrap items-center gap-2">
                    <div class="flex flex-row items-center gap-1">
                        <ViewsIcon/>
                        <p>{views}</p>
                    </div>
                    <p>{"•"}</p>
                    <div class="flex flex-row items-center gap-1">
                        <LikeIcon/>
                        <p>{likes}</p>
                    </div>
                    <p>{"•"}</p>
                    <div class="flex flex-row items-center gap-1">
                        <CalendarIcon/>
                        <p>{published}</p>
                    </div>
                </div>

                <div class="mt-4 flex w-full flex-row items-center justify-between space-x-4">
                    <div class="flex flex-row space-x-4">
                        <img
                            on:load=move |_| img_loaded.set(true)
                            src=author_thumb_url
                            class=image_classes
                        />
                        <div class="flex flex-col space-y-2">
                            <p class="text-xl font-semibold">{author}</p>
                            <button class="btn btn-primary btn-xs">Subscribe</button>
                        </div>
                    </div>
                    <div class="flex flex-row items-end space-x-2">
                        <button class="btn btn-circle btn-primary btn-outline">
                            <DownloadIcon/>
                        </button>
                        <button class="btn btn-circle btn-primary btn-outline">
                            <ShareIcon/>
                        </button>
                        <button class="btn btn-circle btn-primary btn-outline">
                            <PlaylistAddIcon/>
                        </button>
                    </div>
                </div>
            </div>
        </div>
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

