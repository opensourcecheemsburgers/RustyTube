use invidious::{Comment, Replies};
use leptos::{
	expect_context, Action, Resource, RwSignal, SignalGet, SignalSet,
};
use leptos_router::create_query_signal;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

#[derive(Clone, PartialEq, Eq)]
pub struct RepliesResourceArgs {
	server: String,
	locale: RustyTubeLocale,
	video_id: String,
	replies_vec: RwSignal<Vec<Comment>>,
	continuation: RwSignal<Option<String>>,
}

impl RepliesResourceArgs {
	pub fn new(
		replies_vec: RwSignal<Vec<Comment>>,
		continuation: RwSignal<Option<String>>,
	) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			video_id: create_query_signal("id").0.get().unwrap_or_default(),
			replies_vec,
			continuation,
		}
	}
}

#[derive(Clone, Copy)]
pub struct RepliesResource {
	pub resource: Resource<RepliesResourceArgs, Result<(), RustyTubeError>>,
	pub fetch_more: Action<RepliesResourceArgs, Result<(), RustyTubeError>>,
}

impl RepliesResource {
	pub fn initialise(args: RepliesResourceArgs) -> Self {
		Self {
			resource: Resource::local(move || args.clone(), fetch_replies),
			fetch_more: Action::new(|args: &RepliesResourceArgs| {
				fetch_replies(args.clone())
			}),
		}
	}
}

async fn fetch_replies(
	args: RepliesResourceArgs,
) -> Result<(), RustyTubeError> {
	if let Some(token) = args.continuation.get() {
		if let Ok(mut replies) = Replies::fetch_replies(
			args.server.as_str(),
			args.video_id.as_str(),
			token.as_str(),
			args.locale.to_invidious_lang(),
		)
		.await
		{
			args.continuation.set(replies.continuation);
			let mut temp = args.replies_vec.get();
			temp.append(replies.comments.as_mut());
			args.replies_vec.set(temp);
		}
	}
	Ok(())
}
