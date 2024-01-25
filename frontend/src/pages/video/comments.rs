use invidious::{Comment, Comments, Replies};
use leptos::*;
use num_format::{Locale, ToFormattedString};

use crate::{
	components::FerrisError,
	contexts::ServerCtx,
	icons::{LikeIcon, RepliesIcon},
	utils::{get_current_video_query_signal, VideoQuerySignal},
};

#[component]
pub fn CommentsSection() -> impl IntoView {
	let server = expect_context::<ServerCtx>().0 .0;

	let video_id = get_current_video_query_signal();

	let comments_resource = create_resource(
		move || (server.get(), video_id.0.get().unwrap_or_default()),
		|(server, id)| async move { Comments::fetch_comments(&server, &id, None).await },
	);

	view! {
		<Suspense fallback=move || {
			view! { <CommentsSectionPlaceholder/> }
		}>
			{move || {
				comments_resource
					.get()
					.map(|comments_result| {
						match comments_result {
							Ok(comments) => view! { <CommentsSectionContent comments=comments/> },
							Err(err) => view! { <FerrisError error=err/> },
						}
					})
			}}

		</Suspense>
	}
}

#[component]
pub fn CommentsSectionContent(comments: Comments) -> impl IntoView {
	let server = expect_context::<ServerCtx>().0 .0;
	let comments_vec = create_rw_signal(comments.comments);
	let continuation = create_rw_signal(comments.continuation);

	let video_id = get_current_video_query_signal();

	let fetch_comments = create_action(|input: &CommentFetchArgs| {
		let args = input.clone();
		async move {
			fetch_comments(args).await;
		}
	});

	let comment_fetch_args =
		CommentFetchArgs { comments_vec, video_id, continuation, server, fetch_comments };

	let fetch_comments =
		move |_| comment_fetch_args.fetch_comments.dispatch(comment_fetch_args.clone());

	let fetch_more_comments_btn = move || {
		(!comment_fetch_args.comments_vec.get().is_empty()
			&& comment_fetch_args.continuation.get().is_some())
		.then_some(view! {
			<button class="btn btn-primary btn-outline btn-sm" on:click=fetch_comments>
				{"Load more"}
			</button>
		})
		.into_view()
	};

	let comments_view = move || {
		comment_fetch_args
			.comments_vec
			.get()
			.into_iter()
			.map(|comment| view! { <Comment comment=comment/> })
			.collect_view()
	};

	view! {
		<div class="flex flex-col w-full h-[calc(100vh-64px-5rem-128px)] space-y-8">
			<div class="flex flex-col space-y-8">{comments_view}</div>
			{fetch_more_comments_btn}
		</div>
	}
}

#[component]
pub fn Comment(comment: Comment) -> impl IntoView {
	let content = comment.content_html;
	let author = comment.author;
	let author_thumb_url = comment.author_thumbnails.first().cloned().map(|thumb| thumb.url);
	let published = comment.published_text;
	let likes = comment.likes.to_formatted_string(&Locale::en);
	let reply_count = comment.replies_info.clone().map_or(0, |replies| replies.replies);
	let reply_continuation = comment.replies_info.clone().map(|replies| replies.continuation);

	let replies_visible = create_rw_signal(false);
	let replies_vec = create_rw_signal::<Vec<Comment>>(vec![]);
	let fetch_replies_action = create_action(|input: &ReplyFetchArgs| {
		let args = input.clone();
		async move {
			fetch_replies(args).await;
		}
	});

	let reply_fetch_args = ReplyFetchArgs {
		replies_visible,
		continuation: create_rw_signal(reply_continuation),
		replies_vec,
		server: expect_context::<ServerCtx>().0 .0,
		video_id: get_current_video_query_signal(),
		fetch_replies: fetch_replies_action,
	};

	let toggle_replies_visible = move |_| {
		replies_visible.set(!replies_visible.get());
		if replies_vec.get().is_empty() {
			fetch_replies_action.dispatch(reply_fetch_args.clone());
		}
	};

	let replies_view = move || match reply_count != 0 && replies_visible.get() {
		true => Some(view! { <CommentReplies reply_fetch_args=reply_fetch_args/> }),
		false => None,
	};

	view! {
		<div class="flex flex-col space-y-4 h-max">
			<div class="flex flex-row w-full items-start space-x-4">
				<CommenterIcon url=author_thumb_url.unwrap_or_default()/>
				<div class="flex flex-col text-sm">
					<div class="flex flex-row gap-1">
						<p class="font-semibold">{author}</p>
						<p>{"•"}</p>
						<p>{published}</p>
					</div>
					<div class="mt-1" inner_html=content></div>
					<div class="mt-3 flex flex-row gap-1 items-center">
						<LikeIcon/>
						<p>{likes}</p>
						<p>{"•"}</p>
						<div
							class="flex flex-row gap-1 items-center"
							on:click=toggle_replies_visible
						>
							<RepliesIcon/>
							<p>{reply_count}</p>
						</div>
					</div>
				</div>
			</div>
			{replies_view}
		</div>
	}
}

#[component]
pub fn CommenterIcon(url: String) -> impl IntoView {
	let loaded = create_rw_signal(false.to_string());
	let show_image = move |_| loaded.set(true.to_string());

	view! {
		<div
			data-loaded=loaded
			class="data-[loaded=true]:hidden bg-neutral animate-pulse w-6 h-6 rounded-full"
		></div>
		<img
			on:load=show_image
			data-loaded=loaded
			src=url
			class="data-[loaded=false]:hidden w-12 h-12 rounded-full mt-1"
		/>
	}
}

#[component]
pub fn CommentsSectionPlaceholder() -> impl IntoView {
	let comment_placeholders = (0..50).map(|_| view! { <CommentPlaceholder/> }).collect_view();

	view! {
		<div class="flex flex-col w-full h-[calc(100vh-64px-5rem-128px)]">
			{comment_placeholders}
		</div>
	}
}

#[component]
pub fn CommentPlaceholder() -> impl IntoView {
	view! {
		<div class="flex flex-row w-full space-x-4">
			<div class="bg-neutral animate-pulse h-12 w-12 rounded-full"></div>
			<div class="flex flex-col w-full space-y-4">
				<div class="flex flex-row space-x-2 items-center">
					<p class="bg-neutral w-32 h-3 animate-pulse rounded-xl"></p>
					<p class="bg-neutral h-1 w-1 animate-pulse rounded-full"></p>
					<p class="bg-neutral w-20 h-3 animate-pulse rounded-xl"></p>
				</div>
				<div class="flex flex-col space-y-2">
					<p class="bg-neutral w-full h-2 animate-pulse rounded-xl"></p>
					<p class="bg-neutral w-full h-2 animate-pulse rounded-xl"></p>
				</div>
				<div class="flex flex-row space-x-2 items-center">
					<p class="bg-neutral w-8 h-3 animate-pulse rounded-xl"></p>
					<p class="bg-neutral h-1 w-1 animate-pulse rounded-full"></p>
					<p class="bg-neutral w-8 h-3 animate-pulse rounded-xl"></p>
				</div>
			</div>
		</div>
	}
}

#[component]
pub fn CommentReplies(reply_fetch_args: ReplyFetchArgs) -> impl IntoView {
	let fetch_replies = move |_| reply_fetch_args.fetch_replies.dispatch(reply_fetch_args.clone());

	let load_more_replies_btn = move || {
		(!reply_fetch_args.replies_vec.get().is_empty()
			&& reply_fetch_args.continuation.get().is_some())
		.then_some(view! {
			<button class="btn btn-primary btn-outline btn-sm" on:click=fetch_replies>
				{"Load more"}
			</button>
		})
		.into_view()
	};

	let replies = move || {
		reply_fetch_args
			.replies_vec
			.get()
			.into_iter()
			.map(|reply| view! { <Reply comment=reply/> })
			.collect_view()
	};

	view! {
		<div class="pl-2 flex flex-row h-max space-x-3">
			<div class="w-0.5 h-full bg-primary rounded-xl"></div>
			<div class="flex flex-col w-full h-max space-y-4">
				<div class="flex flex-col space-y-4">{replies}</div>
				{load_more_replies_btn}
			</div>
		</div>
	}
}

#[component]
pub fn Reply(comment: Comment) -> impl IntoView {
	let content = comment.content_html;
	let author = comment.author;
	let author_thumb_url = comment.author_thumbnails.first().cloned().map(|thumb| thumb.url);
	let published = comment.published_text;
	let likes = comment.likes.to_formatted_string(&Locale::en);

	view! {
		<div class="flex flex-col space-y-4 h-max">
			<div class="flex flex-row w-full items-start space-x-4">
				<CommenterIcon url=author_thumb_url.unwrap_or_default()/>
				<div class="flex flex-col text-sm">
					<div class="flex flex-row gap-1">
						<p class="font-semibold">{author}</p>
						<p>{"•"}</p>
						<p>{published}</p>
					</div>
					<div class="mt-1" inner_html=content></div>
					<div class="mt-3 flex flex-row gap-1 items-center">
						<LikeIcon/>
						<p>{likes}</p>
						<p>{"•"}</p>
					</div>
				</div>
			</div>
		</div>
	}
}

async fn fetch_comments(args: CommentFetchArgs) {
	if let Some(token) = args.continuation.get() {
		let comments = Comments::fetch_comments(
			args.server.get().as_str(),
			args.video_id.0.get().unwrap().as_str(),
			Some(token.as_str()),
		)
		.await
		.unwrap();
		args.continuation.set(comments.continuation);
		let mut temp = args.comments_vec.get();
		temp.append(comments.comments.clone().as_mut());
		args.comments_vec.set(temp);
	}
}

async fn fetch_replies(args: ReplyFetchArgs) {
	if let Some(token) = args.continuation.get() {
		let replies = Replies::fetch_replies(
			token.as_str(),
			args.server.get().as_str(),
			args.video_id.0.get().unwrap().as_str(),
		)
		.await
		.unwrap();
		args.continuation.set(replies.continuation);
		let mut temp = args.replies_vec.get();
		temp.append(replies.comments.clone().as_mut());
		args.replies_vec.set(temp);
	}
}

#[derive(Clone, Copy)]
pub struct CommentFetchArgs {
	pub continuation: RwSignal<Option<String>>,
	pub comments_vec: RwSignal<Vec<Comment>>,
	pub server: Signal<String>,
	pub video_id: VideoQuerySignal,
	pub fetch_comments: Action<Self, ()>,
}

#[derive(Clone, Copy)]
pub struct ReplyFetchArgs {
	pub replies_visible: RwSignal<bool>,
	pub continuation: RwSignal<Option<String>>,
	pub replies_vec: RwSignal<Vec<Comment>>,
	pub server: Signal<String>,
	pub video_id: VideoQuerySignal,
	pub fetch_replies: Action<Self, ()>,
}
