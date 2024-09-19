use leptos::{
	component, provide_context, view, Children, IntoView, Props, RwSignal,
};

use crate::{
	components::{
		donate_modal::DonateModal, DonateButton, ExpandedCtx, PopularButton,
		SettingsButton, Subs, SubscriptionsButton, TrendingButton,
	},
	icons::FerrisIcon,
};

pub static DRAWER_ID: &str = "sidebar";

#[component]
pub fn Drawer(children: Children) -> impl IntoView {
	provide_context(ExpandedCtx(RwSignal::new(true.to_string())));

	view! {
		<div class="drawer">
			<input id=DRAWER_ID type="checkbox" class="drawer-toggle"/>
			<div class="flex flex-row justify-center items-center drawer-content">
				{children()}
			</div>
			<div class="z-50 drawer-side">
				<label
					for=DRAWER_ID
					aria-label="close sidebar"
					class="drawer-overlay"
				></label>
				<div class="flex flex-col h-full bg-base-200">
					<DrawerHeader/>
					<div class="flex flex-col border-b-[1px] border-b-primary">
						<SubscriptionsButton/>
						<TrendingButton/>
						<PopularButton/>
					</div>
					<Subs/>
					<div class="flex flex-col border-t-[1px] border-t-primary">
						<SettingsButton/>
						<DonateButton/>
					</div>
				</div>
				<DonateModal/>
			</div>
		</div>
	}
}

#[component]
pub fn DrawerHeader() -> impl IntoView {
	view! {
		<label
			for=DRAWER_ID
			class="flex flex-row flex-nowrap btn btn-ghost rtl:gap-x-2"
		>
			<FerrisIcon/>
			<p class="text-xl font-medium normal-case font-display">
				{"RustyTube"}
			</p>
		</label>
	}
}
