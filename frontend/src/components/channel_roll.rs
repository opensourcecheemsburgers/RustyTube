use leptos::*;

use crate::{
	resources::SubscriptionsCtx,
	utils::{go_to, i18n},
};

#[component]
pub fn ChannelRoll(
	channel: String,
	channel_id: String,
	sub_count: String,
	image_url: String,
) -> impl IntoView {
	let subs_ctx = expect_context::<SubscriptionsCtx>();

	let channel = StoredValue::new(channel);
	let channel_id = StoredValue::new(channel_id);
	let image_url = StoredValue::new(image_url);
	let sub_count = StoredValue::new(sub_count);

	let img_loaded = RwSignal::new(false);
	let image_classes = move || match img_loaded.get() {
		true => "h-16 w-16 rounded-full".to_string(),
		false => "h-16 w-16 animate-pulse rounded-full bg-neutral".to_string(),
	};

	let add_sub = Action::new(|args: &AddSubArgs| add_to_subs(args.clone()));
	let remove_sub = Action::new(|args: &RemoveSubArgs| remove_from_subs(args.clone()));

	move || match subs_ctx
		.0
		.get()
		.channels
		.into_iter()
		.find(|sub| sub.id.eq_ignore_ascii_case(&channel_id.get_value()))
		.is_some()
	{
		true => view! {
			<div class="mt-2 flex w-full flex-row items-center justify-between gap-x-4">
				<div class="flex flex-row items-center gap-x-4">
					<img
						on:click=move |_| go_to(format!("/channel?id={}", channel_id.get_value()))
						on:load=move |_| img_loaded.set(true)
						src=image_url.get_value()
						class=image_classes
					/>
					<div class="flex flex-col space-y-2">
						<p class="text-lg md:text-xl font-semibold">{channel.get_value()}</p>
						<button
							on:click=move |_| {
								remove_sub.dispatch((channel_id.get_value(), subs_ctx))
							}

							class="btn btn-primary btn-xs w-32"
						>
							<div class="flex flex-row justify-between gap-3">
								<p>{i18n("channel.subscribed")}</p>
								<p>{sub_count.get_value()}</p>
							</div>
						</button>
					</div>
				</div>
			</div>
		},
		false => view! {
			<div class="flex flex-row gap-x-4">
				<img
					on:click=move |_| go_to(format!("/channel?id={}", channel_id.get_value()))
					on:load=move |_| img_loaded.set(true)
					src=image_url.get_value()
					class=image_classes
				/>
				<div class="flex flex-col space-y-2">
					<p class="text-lg md:text-xl font-semibold">{channel.get_value()}</p>
					<button
						on:click=move |_| {
							add_sub
								.dispatch((channel.get_value(), channel_id.get_value(), subs_ctx))
						}

						class="btn btn-primary btn-xs w-32"
					>
						<div class="flex flex-row justify-between gap-3">
							<p>{i18n("channel.subscribe")}</p>
							<p>{sub_count.get_value()}</p>
						</div>
					</button>
				</div>
			</div>
		},
	}
}

pub type AddSubArgs = (String, String, SubscriptionsCtx);
pub type RemoveSubArgs = (String, SubscriptionsCtx);

async fn add_to_subs(args: (String, String, SubscriptionsCtx)) {
	args.2.add_subscription(&args.1, &args.0).await;
}

async fn remove_from_subs(args: (String, SubscriptionsCtx)) {
	args.1.remove_subscription(&args.0).await;
}
