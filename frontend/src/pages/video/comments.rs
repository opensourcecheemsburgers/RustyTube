use invidious::Comment;
use leptos::*;
use num_format::ToFormattedString;

use crate::{
	components::FerrisError,
	contexts::RegionConfigCtx,
	icons::{LikeIcon, RepliesIcon},
	resources::{CommentsResource, CommentsResourceArgs, RepliesResource, RepliesResourceArgs},
	utils::i18n,
};

#[component]
pub fn CommentsSection() -> impl IntoView {
	let comments_vec = RwSignal::new(vec![]);
	let continuation = RwSignal::new(None);
	let args = CommentsResourceArgs::new(comments_vec, continuation);
	let comments_resource = CommentsResource::initialise(args);

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
							Ok(_) => {
								view! {
									<div class="flex flex-col w-full h-[calc(100vh-64px-5rem-128px)] space-y-8">
										<div class="flex flex-col space-y-8">
											<For
												each=move || comments_vec.get()
												key=|comment| comment.id.clone()
												let:comment
											>
												<Comment comment=comment/>
											</For>
										</div>
										<Show when=move || continuation.get().is_some()>
											<button
												class="btn btn-primary btn-outline btn-sm"
												on:click=move |_| {
													comments_resource.fetch_more.dispatch(args)
												}
											>

												{i18n("general.load_more")}
											</button>
										</Show>
									</div>
								}
									.into_view()
							}
							Err(err) => view! { <FerrisError error=err/> },
						}
					})
			}}

		</Suspense>
	}
}

#[component]
pub fn Comment(comment: Comment) -> impl IntoView {
	let locale = expect_context::<RegionConfigCtx>().locale_slice.0;

	let content = comment.content_html;
	let author = comment.author;
	let author_thumb_url = comment.author_thumbnails.first().cloned().map(|thumb| thumb.url);
	let published = comment.published_text;
	let likes = move || comment.likes.to_formatted_string(&locale.get().to_num_fmt());
	let reply_count = comment.replies_info.clone().map_or(0, |replies| replies.replies);
	let reply_continuation = comment.replies_info.clone().map(|replies| replies.continuation);

	let replies_vec = RwSignal::new(vec![]);
	let continuation = RwSignal::new(reply_continuation);
	let args = RepliesResourceArgs::new(replies_vec, continuation);
	let replies = RepliesResource::initialise(args);

	let replies_visible = RwSignal::new(false);

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
							on:click=move |_| replies_visible.set(!replies_visible.get())
						>
							<RepliesIcon/>
							<p>{reply_count}</p>
						</div>
					</div>
				</div>
			</div>
			<Show when=move || { reply_count != 0 && replies_visible.get() }>
				<div class="pl-2 flex flex-row h-max space-x-3">
					<div class="w-0.5 h-full bg-primary rounded-xl"></div>
					<div class="flex flex-col w-full h-max space-y-4">
						<div class="flex flex-col space-y-4">
							<For
								each=move || replies_vec.get()
								key=|reply| reply.id.clone()
								let:reply
							>
								<Reply reply=reply/>
							</For>
						</div>
						<button
							class="btn btn-primary btn-outline btn-sm"
							on:click=move |_| replies.fetch_more.dispatch(args)
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
pub fn Reply(reply: Comment) -> impl IntoView {
	let author_thumb_url = reply.author_thumbnails.first().cloned().map(|thumb| thumb.url);
	let likes = reply.likes.to_formatted_string(
		&expect_context::<RegionConfigCtx>().locale_slice.0.get().to_num_fmt(),
	);

	view! {
		<div class="flex flex-col space-y-4 h-max">
			<div class="flex flex-row w-full items-start space-x-4">
				<CommenterIcon url=author_thumb_url.unwrap_or_default()/>
				<div class="flex flex-col text-sm">
					<div class="flex flex-row gap-1">
						<p class="font-semibold">{reply.author}</p>
						<p>{"•"}</p>
						<p>{reply.published}</p>
					</div>
					<div class="mt-1" inner_html=reply.content></div>
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
