use invidious::{Trending, TrendingCategory, TrendingCategory::*, CountryCode};
use leptos::*;
use rustytube_error::RustyTubeError;

use crate::components::{VideoPreviewCard, VideoPreviewCardPlaceholderArray, FerrisError};
use crate::contexts::ServerCtx;
use crate::pages::home::homepage::{HomepageSection, HomepageSectionTitle};

#[component]
pub fn TrendingSection(cx: Scope) -> impl IntoView {
    let category = create_rw_signal(cx, Default);
    let server = expect_context::<ServerCtx>(cx).0.0;
    let trending_resource = create_resource(
        cx,
        move || (server.get(), category.get()),
        |(server, category)| async move {
            Trending::fetch_trending(&server, category, CountryCode::IE).await
        },
    );

    // let trending_content_view = move || trending_resource.read(cx).map(|trending_videos_res| {
    //     match trending_videos_res {
    //         Ok(trending) => view! {cx, <TrendingVideos trending=trending />},
    //         Err(err) => view! {cx, <FerrisError error=err/>}
    //     }
    // }) ;

    view! {cx,
        <HomepageSection>
                <HomepageSectionTitle title={"Trending".to_string()}/>
                <TrendingHeader category=category />
                <Suspense fallback=move || view! {cx, <VideoPreviewCardPlaceholderArray />}>
                    {
                        move || trending_resource.read(cx).map(|trending_videos_res| {
                            match trending_videos_res {
                                Ok(trending) => view! {cx, <TrendingVideos trending=trending />},
                                Err(err) => view! {cx, <FerrisError error=err/>}
                            }
                        })
                    }
                </Suspense>
        </HomepageSection>
    }
}

#[component]
pub fn TrendingHeader(cx: Scope, category: RwSignal<TrendingCategory>) -> impl IntoView {
    let header_btn_classes = "btn btn-sm btn-outline font-normal normal-case rounded-lg";

    view! {cx,
        <div class="pl-4 flex flex-row gap-x-3">
            <button on:click=move |_| category.set(Default) class=header_btn_classes>All</button>
            <button on:click=move |_| category.set(Music) class=header_btn_classes>Music</button>
            <button on:click=move |_| category.set(Gaming) class=header_btn_classes>Gaming</button>
            <button on:click=move |_| category.set(Movies) class=header_btn_classes>Movies</button>
       </div>
    }
}

#[component]
pub fn TrendingVideos(cx: Scope, trending: Trending) -> impl IntoView {
    let trending_videos_view = trending.videos.into_iter().map(|trending_video| view!
        { cx,
            <VideoPreviewCard
                video_id=trending_video.id
                title=trending_video.title
                author=trending_video.author
                views=trending_video.views
                published=trending_video.published_text
                thumbnail_url=trending_video.thumbnails.get(3).cloned().unwrap_or_default().url.clone()
            />
        }).collect_view(cx);

    view! {cx,
        <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-4rem-128px)] pb-12 overflow-y-auto scroll-smooth">
            { trending_videos_view }
        </div>
    }
}

// fn fetch_trending_resource(cx: Scope, category: RwSignal<TrendingCategory>) -> TrendingVideosResource {
//     let server = expect_context::<ServerCtx>(cx).0.0;

//     create_resource(
//         cx,
//         move || (server.get(), category.get()),
//         |(server, category)| async move {
//             Trending::fetch_trending(&server, category, CountryCode::IE).await
//         },
//     )
// }

type TrendingVideosResource = Resource<TrendingVideosResourceArgs, Result<Trending, RustyTubeError>>;
type TrendingVideosResourceArgs = (String, TrendingCategory);
