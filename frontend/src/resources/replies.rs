use invidious::{Comment, Replies};
use leptos::*;
use leptos_router::create_query_signal;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RepliesResourceArgs {
	server: Signal<String>,
	locale: Signal<RustyTubeLocale>,
	video_id: Memo<Option<String>>,
	replies_vec: RwSignal<Vec<Comment>>,
	continuation: RwSignal<Option<String>>,
}

impl RepliesResourceArgs {
	pub fn new(
		replies_vec: RwSignal<Vec<Comment>>,
		continuation: RwSignal<Option<String>>,
	) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0,
			locale: expect_context::<RegionConfigCtx>().locale_slice.0,
			video_id: create_query_signal("id").0,
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
		RepliesResource {
			resource: Resource::new(move || args, move |args| fetch_replies(args)),
			fetch_more: Action::new(|args: &RepliesResourceArgs| fetch_replies(args.clone())),
		}
	}
}

async fn fetch_replies(args: RepliesResourceArgs) -> Result<(), RustyTubeError> {
	if let Some(token) = args.continuation.get() {
		let replies = Replies::fetch_replies(
			args.server.get().as_str(),
			args.video_id.get().unwrap().as_str(),
			token.as_str(),
			&args.locale.get().to_invidious_lang(),
		)
		.await
		.unwrap();
		args.continuation.set(replies.continuation);
		let mut temp = args.replies_vec.get();
		temp.append(replies.comments.clone().as_mut());
		args.replies_vec.set(temp);
	}
	Ok(())
}
