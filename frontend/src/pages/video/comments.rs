use invidious::{Comment, Comments};
use leptos::*;
use num_format::ToFormattedString;
use phosphor_leptos::{Chat, IconWeight, ThumbsUp};

use crate::{
	components::FerrisError,
	contexts::RegionConfigCtx,
	resources::{
		CommentsAction, CommentsActionArgs, CommentsResource,
		CommentsResourceArgs, RepliesResource, RepliesResourceArgs,
	},
	utils::i18n,
};

#[component]
pub fn CommentsSection() -> impl IntoView {
	let comments_resource = CommentsResource::initialise();

	view! {
		<Suspense fallback=move || {
			view! { <CommentsSectionPlaceholder/> }
		}>

			{move || {
				comments_resource
					.resource
					.get()
					.map(|comments| {
						match comments {
							Ok(comments) => {
								view! { <CommentsSectionInner comments=comments/> }
							}
							Err(err) => {
								view! { <FerrisError error=err/> }
							}
						}
					})
			}}

		</Suspense>
	}
}

#[component]
pub fn CommentsSectionInner(comments: Comments) -> impl IntoView {
	let comments_vec = RwSignal::new(comments.comments);
	let continuation = RwSignal::new(comments.continuation);
	let comments_action = CommentsAction::new();

	view! {
		<div class="flex flex-col space-y-8 w-full h-[calc(100vh-64px-5rem-128px)]">
			<div class="flex flex-col space-y-8">
				<For
					each=move || comments_vec.get()
					key=|comment: &Comment| comment.id.clone()
					let:comment
				>
					<Comment comment=comment/>
				</For>
			</div>
			<Show when=move || continuation.get().is_some()>
				<button
					class="btn btn-primary btn-outline btn-sm"
					on:click=move |_| {
						comments_action
							.action
							.dispatch(
								CommentsActionArgs::get(comments_vec, continuation),
							);
					}
				>

					{i18n("general.load_more")}
				</button>
			</Show>
		</div>
	}
}

#[component]
pub fn Comment(comment: Comment) -> impl IntoView {
	let locale = expect_context::<RegionConfigCtx>().locale_slice.0;

	let content = comment.content_html;
	let author = comment.author;
	let author_thumb_url =
		comment.author_thumbnails.first().cloned().map(|thumb| thumb.url);
	let published = comment.published_text;
	let likes =
		move || comment.likes.to_formatted_string(&locale.get().to_num_fmt());
	let reply_count =
		comment.replies_info.clone().map_or(0, |replies| replies.replies);
	let reply_continuation =
		comment.replies_info.map(|replies| replies.continuation);

	let replies_vec = RwSignal::new(vec![]);
	let continuation = RwSignal::new(reply_continuation);
	let args = RepliesResourceArgs::new(replies_vec, continuation);
	let args_button = args.clone();
	let args_load_button = args.clone();
	let replies = RepliesResource::initialise(args);

	let replies_visible = RwSignal::new(false);

	view! {
		<div class="flex flex-col space-y-4 h-max">
			<div class="flex flex-row gap-x-4 items-start w-full">
				<CommenterIcon url=author_thumb_url.unwrap_or_default()/>

				<div class="flex flex-col text-sm">
					<div class="flex flex-row gap-1">
						<p class="font-semibold">{author}</p>
						<p>{"•"}</p>
						<p>{published}</p>
					</div>
					<div class="mt-1" inner_html=content></div>
					<div class="flex flex-row gap-1 items-center mt-3">
						<ThumbsUp
							weight=IconWeight::Regular
							class="w-4 h-4 base-content"
						/>
						<p>{likes}</p>
						<p>{"•"}</p>
						<div
							class="flex flex-row gap-1 items-center"
							on:click=move |_| {
								replies.fetch_more.dispatch(args_button.clone());
								replies_visible.set(!replies_visible.get());
							}
						>

							<Chat
								weight=IconWeight::Regular
								class="w-4 h-4 base-content"
							/>
							<p>{reply_count}</p>
						</div>
					</div>
				</div>
			</div>
			<Show when=move || { reply_count != 0 && replies_visible.get() }>
				<div class="flex flex-row gap-x-3 pl-2 h-max">
					<div class="w-0.5 h-full rounded-xl bg-primary"></div>
					<div class="flex flex-col space-y-4 w-full h-max">
						{move || {
							replies
								.resource
								.get()
								.map(|replies| {
									view! {
										<div class="flex flex-col space-y-4">
											<For
												each=move || replies_vec.get()
												key=|reply| reply.id.clone()
												let:reply
											>
												<Reply reply=reply/>
											</For>
										</div>
									}
								})
						}}
						<button
							class:hidden=move || continuation.get().is_none()
							class="btn btn-primary btn-outline btn-sm"
							on:click={
								let args = args_load_button.clone();
								move |_| replies.fetch_more.dispatch(args.clone())
							}
						>

							{i18n("general.load_more")}
						</button>
					</div>
				</div>
			</Show>
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
			class="hidden w-6 h-6 rounded-full animate-pulse bg-neutral md:data-[loaded=false]:!flex"
		></div>
		<img
			on:load=show_image
			data-loaded=loaded
			src=url
			class="hidden mt-1 w-12 h-12 rounded-full md:data-[loaded=true]:flex"
		/>
	}
}

#[component]
pub fn Reply(reply: Comment) -> impl IntoView {
	let author_thumb_url =
		reply.author_thumbnails.first().cloned().map(|thumb| thumb.url);
	let likes = reply.likes.to_formatted_string(
		&expect_context::<RegionConfigCtx>().locale_slice.0.get().to_num_fmt(),
	);

	view! {
		<div class="flex flex-col space-y-4 h-max">
			<div class="flex flex-row gap-x-4 items-start w-full">
				<CommenterIcon url=author_thumb_url.unwrap_or_default()/>
				<div class="flex flex-col text-sm">
					<div class="flex flex-row gap-1">
						<p class="font-semibold">{reply.author}</p>
						<p>{"•"}</p>
						<p>{reply.published}</p>
					</div>
					<div class="mt-1" inner_html=reply.content></div>
					<div class="flex flex-row gap-1 items-center mt-3">
						<ThumbsUp
							weight=IconWeight::Regular
							class="w-4 h-4 base-content"
						/>
						<p>{likes}</p>
						<p>{"•"}</p>
					</div>
				</div>
			</div>
		</div>
	}
}

#[component]
pub fn CommentsSectionPlaceholder() -> impl IntoView {
	let comment_placeholders =
		(0..50).map(|_| view! { <CommentPlaceholder/> }).collect_view();

	view! {
		<div class="flex flex-col w-full h-[calc(100vh-64px-5rem-128px)]">
			{comment_placeholders}
		</div>
	}
}

#[component]
pub fn CommentPlaceholder() -> impl IntoView {
	view! {
		<div class="flex flex-row gap-x-4 w-full">
			<div class="w-12 h-12 rounded-full animate-pulse bg-neutral"></div>
			<div class="flex flex-col space-y-4 w-full">
				<div class="flex flex-row gap-x-2 items-center">
					<p class="w-32 h-3 rounded-xl animate-pulse bg-neutral"></p>
					<p class="w-1 h-1 rounded-full animate-pulse bg-neutral"></p>
					<p class="w-20 h-3 rounded-xl animate-pulse bg-neutral"></p>
				</div>
				<div class="flex flex-col space-y-2">
					<p class="w-full h-2 rounded-xl animate-pulse bg-neutral"></p>
					<p class="w-full h-2 rounded-xl animate-pulse bg-neutral"></p>
				</div>
				<div class="flex flex-row gap-x-2 items-center">
					<p class="w-8 h-3 rounded-xl animate-pulse bg-neutral"></p>
					<p class="w-1 h-1 rounded-full animate-pulse bg-neutral"></p>
					<p class="w-8 h-3 rounded-xl animate-pulse bg-neutral"></p>
				</div>
			</div>
		</div>
	}
}
