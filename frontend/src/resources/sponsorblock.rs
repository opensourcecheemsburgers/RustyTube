use leptos::*;
use rustytube_error::RustyTubeError;
use sponsorblock_rs::{Query, Response, Segment};

#[derive(Clone, Copy)]
pub struct SponsorBlockResource {
	pub resource: RwSignal<Option<Resource<String, Result<Option<Response>, RustyTubeError>>>>,
}

impl SponsorBlockResource {
	pub fn set_video(&self, video_id: Memo<Option<String>>) {
		self.resource.set(Some(Resource::new(
			move || video_id.get().unwrap_or_default(),
			move |id: String| fetch_sponsorblock_segments(id),
		)));
	}

	pub fn empty() -> Self {
		SponsorBlockResource { resource: RwSignal::new(None) }
	}

	pub fn get_segments(&self) -> Option<Vec<Segment>> {
		Some(self.resource.get()?.get()?.ok()??.segments)
	}
}

async fn fetch_sponsorblock_segments(id: String) -> Result<Option<Response>, RustyTubeError> {
	Ok(Query::build(id).send_query().await.unwrap())
}
