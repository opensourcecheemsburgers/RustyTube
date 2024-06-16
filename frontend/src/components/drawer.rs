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
			<div class="drawer-content flex flex-row items-center justify-center">
				{children()}
			</div>
			<div class="drawer-side z-50">
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
			class="btn btn-ghost flex flex-row flex-nowrap rtl:gap-x-2"
		>
			<FerrisIcon/>
			<p class="normal-case font-display font-medium text-xl">
				{"RustyTube"}
			</p>
		</label>
	}
}
