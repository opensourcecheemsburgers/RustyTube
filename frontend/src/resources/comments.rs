use invidious::{Comment, Comments};
use leptos::{
	expect_context, Action, Resource, RwSignal, SignalGet, SignalSet,
};
use leptos_router::create_query_signal;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

#[derive(Clone, PartialEq, Eq)]
pub struct CommentsResourceArgs {
	server: String,
	locale: RustyTubeLocale,
	video_id: String,
}

impl CommentsResourceArgs {
	pub fn new() -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			video_id: create_query_signal("id").0.get().unwrap_or_default(),
		}
	}
}

#[derive(Clone, Copy)]
pub struct CommentsResource {
	pub resource:
		Resource<CommentsResourceArgs, Result<Comments, RustyTubeError>>,
}

impl CommentsResource {
	pub fn initialise() -> Self {
		Self {
			resource: Resource::local(
				CommentsResourceArgs::new,
				fetch_comments,
			),
		}
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct CommentsActionArgs {
	server: String,
	locale: RustyTubeLocale,
	video_id: String,
	comments_vec: RwSignal<Vec<Comment>>,
	continuation: RwSignal<Option<String>>,
}

impl CommentsActionArgs {
	pub fn get(
		comments_vec: RwSignal<Vec<Comment>>,
		continuation: RwSignal<Option<String>>,
	) -> Self {
		Self {
			server: expect_context::<NetworkConfigCtx>().server_slice.0.get(),
			locale: expect_context::<RegionConfigCtx>().locale_slice.0.get(),
			video_id: create_query_signal("id").0.get().unwrap_or_default(),
			comments_vec,
			continuation,
		}
	}
}

#[derive(Clone, Copy)]
pub struct CommentsAction {
	pub action: Action<CommentsActionArgs, Result<(), RustyTubeError>>,
}

impl CommentsAction {
	pub fn new() -> Self {
		Self {
			action: Action::new(|args: &CommentsActionArgs| {
				fetch_more_comments(args.clone())
			}),
		}
	}
}

async fn fetch_comments(
	args: CommentsResourceArgs,
) -> Result<Comments, RustyTubeError> {
	Comments::fetch_comments(
		args.server.as_str(),
		args.video_id.as_str(),
		None,
		args.locale.to_invidious_lang(),
	)
	.await
}

async fn fetch_more_comments(
	args: CommentsActionArgs,
) -> Result<(), RustyTubeError> {
	if args.continuation.get().is_some() || args.comments_vec.get().is_empty() {
		let mut comments = Comments::fetch_comments(
			args.server.as_str(),
			args.video_id.as_str(),
			args.continuation.get().as_deref(),
			args.locale.to_invidious_lang(),
		)
		.await?;
		args.continuation.set(comments.continuation);
		let mut temp = args.comments_vec.get();
		temp.append(comments.comments.as_mut());
		args.comments_vec.set(temp);
	}
	Ok(())
}
