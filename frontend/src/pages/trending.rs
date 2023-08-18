use config::Config;
use invidious::channel::Channel;
use invidious::common::CommonVideo;
use invidious::hidden::CountryCode;
use invidious::universal::{Trending, TrendingCategory, TrendingCategory::*};
use leptos::*;
use num_format::{Locale, ToFormattedString};
use crate::components::{VideoPreviewCard, VideoPreviewCardPlaceholderArray};
use crate::ServerCtx;

#[component]
pub fn Trending(cx: Scope) -> impl IntoView {
    let category = create_rw_signal(cx, Default);

    view! {cx,
        <div class="w-screen flex justify-center mt-4">
            <div class="w-[90%] flex flex-col gap-y-8">
                <h1 class="pl-4 font-semibold text-2xl">
                {"Trending"}
                </h1>
                <TrendingHeader category=category.write_only() />
                <TrendingVideos category=category.read_only()/>
            </div>
        </div>
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
pub fn TrendingVideos(cx: Scope, category: ReadSignal<TrendingCategory>) -> impl IntoView {
    let server = expect_context::<ServerCtx>(cx).0.0;

    let trending_videos = create_resource(
        cx,
        move || (server.get(), category.get()),
        |(server, category)| async move {
            // &config.network.server
            let trending_res = Trending::fetch_trending(&server, category, CountryCode::IE)
                .await;

            trending_res
        },
    );
    
    view! {cx,
        {
            move ||
                match trending_videos.read(cx) {
                    Some(result) => match result {
                        Ok(trending) => {
                            view! {cx,
                            <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-1rem-128px)] pb-12 overflow-y-scroll">
                                {
                                    trending.videos.into_iter().map(|trending_video| view!
                                        { cx,
                                            <VideoPreviewCard video={trending_video} />
                                        }
                                    ).collect_view(cx)
                                }
                            </div>
                            }.into_view(cx)
                        },
                        Err(err) => view! {cx, <div class="text-2xl font-mono">{err.description}</div>}.into_view(cx)
                    },
                    None => view! {cx, <VideoPreviewCardPlaceholderArray />}
                }
        }
    }
}
