use leptos::*;
use leptos_router::{ParamsMap, use_params, use_params_map};
use invidious::{Comments, Video};
use rustytube_error::RustyTubeError;

use crate::{components::{Header, ScrollablePage}, contexts::{PlayerStyle, PlayerState}};
use crate::contexts::{ServerCtx};
use crate::pages::video::get_current_video_query_signal;

use super::{video_player::{VideoContainer, VideoPlaceholder}, info::{VideoInfo, VideoInfoPlaceholder}, comments::{CommentsSection, CommentsSectionPlaceholder}, recommended::{RecommendedSection, RecommendedSectionPlaceholder}};

#[component]
pub fn VideoPage() -> impl IntoView {
    let server = expect_context::<ServerCtx>().0.0;
    let id = get_current_video_query_signal();

    let video_resource: VideoResource = create_resource(
                move || (server.get(), id.0.get().unwrap_or_default()),
        |(server, id)| async move {
            let video = Video::fetch_video(&server, &id).await;
            video
        }
    );

    view! {
        <ScrollablePage>
            <div class="flex flex-row space-x-4 px-4">
                <div class="flex flex-col basis-4/6 item-start">
                    <VideoContainer video_resource=video_resource/>
                    <div class="mt-5">
                        <VideoInfo video_resource=video_resource/>
                    </div>
                    <div class="mt-10">
                        <CommentsSection/>
                    </div>
                </div>
                <div class="flex flex-col basis-2/6">
                    <RecommendedSection video_resource=video_resource/>
                </div>
            </div>
        </ScrollablePage>
    }
}

pub type VideoResource = Resource<(String, String), Result<Video, RustyTubeError>>;
