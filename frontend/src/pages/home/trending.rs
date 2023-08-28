use invidious::{Trending, TrendingCategory, TrendingCategory::*, CountryCode};
use leptos::*;
use rustytube_error::RustyTubeError;

use crate::components::{VideoPreviewCard, VideoPreviewCardPlaceholderArray, FerrisError};
use crate::contexts::ServerCtx;
use crate::pages::home::homepage::{HomepageSection, HomepageSectionTitle};

#[component]
pub fn TrendingSection(cx: Scope) -> impl IntoView {
    let category = create_rw_signal(cx, Default);

    view! {cx,
        <HomepageSection>
                <HomepageSectionTitle title={"Trending".to_string()}/>
                <TrendingHeader category=category.write_only() />
                <TrendingContent category=category.read_only() />
        </HomepageSection>
    }
}

#[component]
pub fn TrendingHeader(cx: Scope, category: WriteSignal<TrendingCategory>) -> impl IntoView {
    view! {cx,
        <div class="pl-4 flex flex-row gap-x-3">
            <button on:click=move |_| category.set(Default) class="btn btn-sm btn-outline font-normal normal-case rounded-lg">All</button>
            <button on:click=move |_| category.set(Music) class="btn btn-sm btn-outline font-normal normal-case rounded-lg">Music</button>
            <button on:click=move |_| category.set(Gaming) class="btn btn-sm btn-outline font-normal normal-case rounded-lg">Gaming</button>
            <button on:click=move |_| category.set(Movies) class="btn btn-sm btn-outline font-normal normal-case rounded-lg">Movies</button>
       </div>
    }
}

#[component]
pub fn TrendingContent(cx: Scope, category: ReadSignal<TrendingCategory>) -> impl IntoView {
    let server = expect_context::<ServerCtx>(cx).0.0;

    let trending_videos = create_resource(
        cx,
        move || (server.get(), category.get()),
        |(server, category)| async move {
            Trending::fetch_trending(&server, category, CountryCode::IE).await
        },
    );

    let trending_content_view = move || match trending_videos.read(cx) {
        Some(result) => match result {
            Ok(trending) => view! {cx, <TrendingVideos trending=trending />},
            Err(err) => view! {cx, <TrendingError error=err />}
        },
        None => view! {cx, <VideoPreviewCardPlaceholderArray />}
    };

    view! {cx, { trending_content_view } }
}

#[component]
pub fn TrendingVideos(cx: Scope, trending: Trending) -> impl IntoView {
    let trending_videos_view = trending.videos.into_iter().map(|trending_video| view!
        { cx,
            <VideoPreviewCard
                title=trending_video.title
                author=trending_video.author
                views=trending_video.views
                published=trending_video.published_text
                thumbnail_url=trending_video.thumbnails.first().unwrap().url.clone()
            />
        }).collect_view(cx);

    view! {cx,
        <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-4rem-128px)] pb-12 overflow-y-auto scroll-smooth">
            { trending_videos_view }
        </div>
    }
}

#[component]
pub fn TrendingError(cx: Scope, error: RustyTubeError) -> impl IntoView {
    view! {cx,
        <div class="h-[calc(100vh-64px-1rem-128px)]">
            <FerrisError error=error width=96 />
        </div>
    }
}