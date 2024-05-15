use leptos::*;

use crate::{
	components::{
		donate_modal::DonateModal, DonateButton, ExpandedCtx, PopularButton, SettingsButton, Subs,
		SubscriptionsButton, TrendingButton,
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
				class="overflow-y-scroll dropdown-content p-3 shadow bg-base-300 rounded-xl w-64 max-h-80 h-fit z-10"
			>
				<For each=move || rust_i18n::available_locales!() key=|locale| locale let:locale>
					<li>
						<a
							class="btn btn-xs md:btn-sm btn-ghost h-fit btn-block justify-start text-left"
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
