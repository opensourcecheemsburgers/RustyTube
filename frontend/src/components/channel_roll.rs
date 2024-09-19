use leptos::{
	component, expect_context, view, Action, IntoView, RwSignal, SignalGet,
	SignalSet, StoredValue,
};

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
	let image_classes = move || {
		if img_loaded.get() {
			"h-16 w-16 rounded-full".to_string()
		} else {
			"h-16 w-16 animate-pulse rounded-full bg-neutral".to_string()
		}
	};

	let add_sub = Action::new(|args: &AddSubArgs| add_to_subs(args.clone()));
	let remove_sub =
		Action::new(|args: &RemoveSubArgs| remove_from_subs(args.clone()));

	move || {
		if subs_ctx
			.0
			.get()
			.channels
			.into_iter()
			.any(|sub| sub.id.eq_ignore_ascii_case(&channel_id.get_value()))
		{
			view! {
				<div class="flex flex-row gap-x-4 justify-between items-center mt-2 w-full">
					<div class="flex flex-row gap-x-4 items-center">
						<img
							on:click=move |_| go_to(
								format!("/channel?id={}", channel_id.get_value()),
							)

							on:load=move |_| img_loaded.set(true)
							src=image_url.get_value()
							class=image_classes
						/>
						<div class="flex flex-col space-y-2">
							<p class="text-lg font-semibold md:text-xl">
								{channel.get_value()}
							</p>
							<button
								on:click=move |_| {
									remove_sub.dispatch((channel_id.get_value(), subs_ctx));
								}

								class="w-32 btn btn-primary btn-xs"
							>
								<div class="flex flex-row gap-3 justify-between">
									<p>{i18n("channel.subscribed")}</p>
									<p>{sub_count.get_value()}</p>
								</div>
							</button>
						</div>
					</div>
				</div>
			}
		} else {
			view! {
				<div class="flex flex-row gap-x-4">
					<img
						on:click=move |_| go_to(
							format!("/channel?id={}", channel_id.get_value()),
						)

						on:load=move |_| img_loaded.set(true)
						src=image_url.get_value()
						class=image_classes
					/>
					<div class="flex flex-col space-y-2">
						<p class="text-lg font-semibold md:text-xl">
							{channel.get_value()}
						</p>
						<button
							on:click=move |_| {
								add_sub
									.dispatch((
										channel.get_value(),
										channel_id.get_value(),
										subs_ctx,
									));
							}

							class="w-32 btn btn-primary btn-xs"
						>
							<div class="flex flex-row gap-3 justify-between">
								<p>{i18n("channel.subscribe")}</p>
								<p>{sub_count.get_value()}</p>
							</div>
						</button>
					</div>
				</div>
			}
		}
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
