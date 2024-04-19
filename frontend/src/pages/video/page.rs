use invidious::Video;
use leptos::*;
use rustytube_error::RustyTubeError;

use super::{
	comments::CommentsSection, info::VideoInfo, recommended::RecommendedSection,
	video_player::VideoContainer,
};
use crate::{
	contexts::{NetworkConfigCtx, PlayerState, PlayerStyle, RegionConfigCtx},
	utils::get_current_video_query_signal,
};

#[component]
pub fn VideoPage() -> impl IntoView {
	let locale = expect_context::<RegionConfigCtx>().locale_slice.0;
	let server = expect_context::<NetworkConfigCtx>().server_slice.0;
	let id = get_current_video_query_signal().0;

	let video_resource: VideoResource = create_resource(
		move || (server.get(), id.get().unwrap_or_default(), locale.get().to_invidious_lang()),
		|(server, id, lang)| async move {
			let video = Video::fetch_video(&server, &id, &lang).await;
			video
		},
	);

	let state = PlayerState::init();
	let style = PlayerStyle::init();
	provide_context(state);
	provide_context(style);

	view! {
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
	}
}

pub type VideoResource = Resource<(String, String, String), Result<Video, RustyTubeError>>;
