use leptos::{expect_context, Memo, Resource, RwSignal, SignalGet, SignalSet};
use rustytube_error::RustyTubeError;
use sponsorblock_rs::{Category, Query, Response, Segment};

use crate::contexts::SponsorBlockConfigCtx;

#[allow(clippy::type_complexity)]
#[derive(Clone, Copy)]
pub struct SponsorBlockResource {
	pub resource: RwSignal<
		Option<Resource<String, Result<Option<Response>, RustyTubeError>>>,
	>,
}

impl SponsorBlockResource {
	pub fn set_video(&self, video_id: Memo<Option<String>>) {
		self.resource.set(Some(Resource::local(
			move || video_id.get().unwrap_or_default(),
			move |id: String| fetch_sponsorblock_segments(id),
		)));
	}

	pub fn empty() -> Self {
		Self { resource: RwSignal::new(None) }
	}

	pub fn get_segments(&self) -> Option<Vec<Segment>> {
		Some(self.resource.get()?.get()?.ok()??.segments)
	}
}

async fn fetch_sponsorblock_segments(
	id: String,
) -> Result<Option<Response>, RustyTubeError> {
	let categories = move || {
		let ctx = expect_context::<SponsorBlockConfigCtx>();

		let mut vec = vec![];
		ctx.skip_sponsors.0.get().then(|| vec.push(Category::Sponsor));
		ctx.skip_selfpromos.0.get().then(|| vec.push(Category::SelfPromotion));
		ctx.skip_intros.0.get().then(|| vec.push(Category::Intro));
		ctx.skip_outros.0.get().then(|| vec.push(Category::Outro));
		ctx.skip_interactions.0.get().then(|| vec.push(Category::Interaction));
		ctx.skip_previews.0.get().then(|| vec.push(Category::Preview));
		ctx.skip_irrelevant_music
			.0
			.get()
			.then(|| vec.push(Category::OffTopicMusic));
		ctx.skip_filler.0.get().then(|| vec.push(Category::Filler));
		Some(vec)
	};

	Ok(Query::create(id, None, categories(), None, None)
		.send_query()
		.await
		.expect("Error fetching sponsorblock data."))
}
