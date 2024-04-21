use invidious::{Comment, Comments};
use leptos::*;
use leptos_router::create_query_signal;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CommentsResourceArgs {
	server: Signal<String>,
	locale: Signal<RustyTubeLocale>,
	video_id: Memo<Option<String>>,
	comments_vec: RwSignal<Vec<Comment>>,
	continuation: RwSignal<Option<String>>,
}

impl CommentsResourceArgs {
	pub fn new(
		comments_vec: RwSignal<Vec<Comment>>,
		continuation: RwSignal<Option<String>>,
	) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0,
			locale: expect_context::<RegionConfigCtx>().locale_slice.0,
			video_id: create_query_signal("id").0,
			comments_vec,
			continuation,
		}
	}
}

#[derive(Clone, Copy)]
pub struct CommentsResource {
	pub resource: Resource<CommentsResourceArgs, Result<(), RustyTubeError>>,
	pub fetch_more: Action<CommentsResourceArgs, Result<(), RustyTubeError>>,
}

impl CommentsResource {
	pub fn initialise(args: CommentsResourceArgs) -> Self {
		CommentsResource {
			resource: Resource::new(move || args, move |args| fetch_comments(args)),
			fetch_more: Action::new(|args: &CommentsResourceArgs| fetch_comments(args.clone())),
		}
	}
}

async fn fetch_comments(args: CommentsResourceArgs) -> Result<(), RustyTubeError> {
	if args.continuation.get().is_some() || args.comments_vec.get().len() == 0 {
		let comments = Comments::fetch_comments(
			args.server.get().as_str(),
			args.video_id.get().unwrap().as_str(),
			args.continuation.get().as_deref(),
			&args.locale.get().to_invidious_lang(),
		)
		.await
		.unwrap();
		args.continuation.set(comments.continuation);
		let mut temp = args.comments_vec.get();
		temp.append(comments.comments.clone().as_mut());
		args.comments_vec.set(temp);
	}
	Ok(())
}
