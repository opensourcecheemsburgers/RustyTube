use crate::components::{VideoPreviewCard, VideoPreviewCardPlaceholderArray};
use crate::contexts::ServerCtx;
use invidious::{Popular, TrendingCategory::*};
use leptos::*;
use crate::pages::home::homepage::{HomepageSection, HomepageSectionTitle};

#[component]
pub fn PopularSection(cx: Scope) -> impl IntoView {
    view! {cx,
        <HomepageSection>
            <HomepageSectionTitle title={"Popular".to_string()}/>
            <PopularVideos />
        </HomepageSection>
    }
}

#[component]
pub fn PopularVideos(cx: Scope) -> impl IntoView {
    let server = expect_context::<ServerCtx>(cx).0 .0;

    let popular_videos = create_resource(
        cx,
        move || server.get(),
        |server| async move { Popular::fetch_popular(&server).await },
    );

    view! {cx,
        {
            move ||
                match popular_videos.read(cx) {
                    Some(result) => match result {
                        Ok(popular) => {
                            view! {cx,
                            <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-1rem-128px)] pb-12 overflow-y-scroll">
                                {
                                    popular.items.into_iter().map(|video| view!
                                        { cx,
                                            <VideoPreviewCard
                                                title=video.title
                                                author=video.author
                                                views=video.views
                                                published=video.published_text
                                                thumbnail_url=video.thumbnails.first().unwrap().url.clone()
                                            />
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