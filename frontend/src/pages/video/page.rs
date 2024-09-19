use leptos::{
	component, expect_context, provide_context, view, IntoView, Props, Signal,
	SignalGet,
};
use leptos_router::create_query_signal;

use super::{
	comments::CommentsSection, info::VideoInfo, video_player::VideoContainer,
};
use crate::{
	components::RecommendedSectionCollapsible,
	resources::{SponsorBlockResource, VideoResource},
};

#[component]
pub fn VideoPage() -> impl IntoView {
	provide_context(VideoResource::initialise());

	expect_context::<SponsorBlockResource>()
		.set_video(create_query_signal::<String>("id").0);

	let playlist_query_signal = create_query_signal::<String>("videos").0;
	let playlist_videos = Signal::derive(move || {
		playlist_query_signal.get().map(|videos| {
			videos
				.split(',')
				.map(std::string::ToString::to_string)
				.collect::<Vec<String>>()
		})
	});

	view! {
		<div class="flex flex-row gap-x-4 mb-48 md:px-4">
			<div class="flex flex-col basis-full item-start lg:basis-4/6">
				<VideoContainer/>
				<div class="mt-5">
					<VideoInfo/>
				</div>
				<div class="mt-5 lg:hidden">
					// {move || match playlist_query_signal.get().is_some() {
					// true => ().into_view(),
					// false => view! { <RecommendedSectionCollapsible/> },
					// }}
					<RecommendedSectionCollapsible/>
				</div>
				<div class="mt-5 lg:mt-10">
					<CommentsSection/>
				</div>
			</div>
			<div class="hidden flex-col basis-2/6 lg:!flex">
				<RecommendedSectionCollapsible/>
			</div>
		</div>
	}
}
