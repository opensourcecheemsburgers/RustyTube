use leptos::*;
use leptos_router::Outlet;
use phosphor_leptos::{Chat, IconWeight};

use crate::{
	components::{Drawer, ExpandedCtx, Header, Sidebar, Toaster},
	contexts::{Toast, UiConfigCtx},
};

#[component]
pub fn Page() -> impl IntoView {
	let theme = expect_context::<UiConfigCtx>().theme_slice.0;

	let expanded = RwSignal::new(false.to_string());
	provide_context(ExpandedCtx(expanded));

	view! {
		<div
			data-theme=theme
			class="flex flex-row h-svh min-h-svh w-svw lg:h-screen lg:min-h-screen lg:w-screen"
		>
			<Drawer>
				<Sidebar/>
				<div
					data-expanded=expanded
					class="data-[expanded=false]:w-[calc(100vw-16px)] data-[expanded=true]:w-[calc(100vw-64px)]
					"
				>
					<Header/>
					<div class="bg-base-100 h-[calc(100svh-64px)] min-h-[calc(100svh-64px)] w-full overflow-x-hidden overflow-y-visible scroll-smooth">
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

pub const PAGE_CLASSES: &'static str = "
flex flex-col w-svw h-svw

lg:data-[expanded=false]:w-[calc(100vw-16px)]
lg:data-[expanded=true]:w-[calc(100vw-64px)]
";
