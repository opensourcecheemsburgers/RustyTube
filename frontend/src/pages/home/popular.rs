use crate::components::{VideoPreviewCard, VideoPreviewCardPlaceholderArray, FerrisError};
use crate::contexts::ServerCtx;
use invidious::{Popular, TrendingCategory::*};
use leptos::*;
use crate::pages::home::homepage::{HomepageSection, HomepageSectionTitle};

#[component]
pub fn PopularSection() -> impl IntoView {
    let server = expect_context::<ServerCtx>().0 .0;

    let popular_videos = create_resource(
        move || server.get(),
        |server| async move { Popular::fetch_popular(&server).await },
    );

    view! {       
        <HomepageSection>
            <HomepageSectionTitle title={"Popular".to_string()}/>
            <Suspense fallback=move || view! {<VideoPreviewCardPlaceholderArray />}>
                {
                    move || popular_videos.get().map(|popular_videos_res| {
                        match popular_videos_res {
                            Ok(popular) => view! {<PopularVideos popular=popular/>}.into_view(),
                            Err(err) => view! {<FerrisError error=err/>}
                        }
                    })
                }
            </Suspense>
        </HomepageSection>
    }
}

#[component]
pub fn PopularVideos(popular: Popular) -> impl IntoView {
    let popular_videos_view = popular.items.into_iter().map(|video| view! {
        <VideoPreviewCard
            video_id=video.id
            title=video.title
            author=video.author
            views=video.views
            published=video.published_text
            thumbnail_url=video.thumbnails.get(3).cloned().unwrap_or_default().url.clone()
        />
    }).collect_view();

    view! {
        <div class="flex flex-row flex-wrap gap-y-12 h-[calc(100vh-64px-4rem-128px)] pb-12 overflow-y-auto scroll-smooth">
            { popular_videos_view }
        </div>
    }
}