use leptos::*;

use crate::{
	components::{
		donate_modal::DonateModal, DonateButton, ExpandedCtx, PopularButton,
		SettingsButton, Subs, SubscriptionsButton, TrendingButton,
	},
	icons::FerrisIcon,
};

#[component]
pub fn LocaleDropdown(btn_classes: String) -> impl IntoView {
	let locale_slice = expect_context::<RegionConfigCtx>().locale_slice;

	view! {
		<div class="dropdown dropdown-end">
			<div tabindex="0" role="button" class=btn_classes>
				{move || locale_slice.0.get().human_name()}
			</div>
			<ul
				tabindex="0"
				class="overflow-y-scroll z-10 p-3 w-64 max-h-80 rounded-xl shadow dropdown-content bg-base-300 h-fit"
			>
				<For
					each=move || rust_i18n::available_locales!()
					key=|locale| locale
					let:locale
				>
					<li>
						<a
							class="justify-start text-left btn btn-xs btn-ghost h-fit btn-block md:btn-sm"
							on:click=set_locale
						>
							<p>{locale.human_name()}</p>
						</a>
					</li>
				</For>
			</ul>
		</div>
	}
}
