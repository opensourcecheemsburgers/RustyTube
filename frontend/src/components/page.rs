use leptos::{
	component, expect_context, provide_context, view, IntoView, Props,
	RwSignal, Signal, SignalGet,
};
use leptos_router::Outlet;

use crate::{
	components::{Drawer, ExpandedCtx, Header, Sidebar, Toaster},
	contexts::{RegionConfigCtx, Toast, UiConfigCtx},
};

#[component]
pub fn Page() -> impl IntoView {
	let theme = expect_context::<UiConfigCtx>().theme_slice.0;

	let expanded = RwSignal::new(false.to_string());
	provide_context(ExpandedCtx(expanded));

	let dir_signal = Signal::derive(move || {
		if expect_context::<RegionConfigCtx>()
			.locale_slice
			.0
			.get()
			.is_rtl_lang()
		{
			"rtl"
		} else {
			"ltr"
		}
	});

	view! {
		<div
			dir=dir_signal
			data-theme=theme
			class="flex lg:w-screen lg:h-screen lg:min-h-screen h-svh min-h-svh w-svw"
		>
			<Drawer>
				<Sidebar/>
				<div
					data-expanded=expanded
					class="data-[expanded=false]:w-[calc(100vw-16px)] data-[expanded=true]:w-[calc(100vw-64px)]"
				>
					<Header/>
					<div class="overflow-x-hidden overflow-y-visible w-full bg-base-100 h-[calc(100svh-64px)] min-h-[calc(100svh-64px)] scroll-smooth">
						<Outlet/>
						<RustyTubeToaster/>
					</div>
				</div>
			</Drawer>
		</div>
	}
}

#[component]
fn RustyTubeToaster() -> impl IntoView {
	view! {
		<Toaster
			default_view=move |toast: Toast| {
				view! {
					<div class="alert">
						<span>{toast.message.get_value()}</span>
					</div>
				}
			}

			info_view=move |toast: Toast| {
				view! {
					<div class="alert alert-info">
						<span>{toast.message.get_value()}</span>
					</div>
				}
			}

			warning_view=move |toast: Toast| {
				view! {
					<div class="alert alert-warning">
						<span>{toast.message.get_value()}</span>
					</div>
				}
			}

			error_view=move |toast: Toast| {
				view! {
					<div class="alert alert-error">
						<span>{toast.message.get_value()}</span>
					</div>
				}
			}
		/>
	}
}

pub const PAGE_CLASSES: &str = "
flex flex-col w-dvw h-dvh \
\
lg:data-[expanded=false]:w-[calc(100vw-16px)]
lg:data-[expanded=true]:w-[calc(100vw-64px)]
";
