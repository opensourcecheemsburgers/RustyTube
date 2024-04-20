use leptos::*;
use leptos_router::Outlet;

use crate::{
	components::{Header, Sidebar, Toaster},
	contexts::{Toast, UiConfigCtx},
};

#[component]
pub fn Page() -> impl IntoView {
	let theme = expect_context::<UiConfigCtx>().theme_slice.0;

	let expanded = create_rw_signal(true.to_string());
	provide_context(expanded);

	view! {
		<div
			data-theme=theme
			class="flex flex-row min-h-screen max-h-screen bg-base-100 min-w-screen max-w-screen"
		>
			<Sidebar/>
			<div data-expanded=expanded class=PAGE_CLASSES>
				<Header/>
				<div class="min-h-[calc(100vh-4rem)] max-h-[calc(100vh-4rem)] min-w-screen max-w-screen bg-base-100 overflow-y-auto no-scrollbar">
					<Outlet/>
					<RustyTubeToaster/>
				</div>
			</div>
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
flex flex-col

data-[expanded=false]:w-[calc(100vw-16px)]
data-[expanded=true]:w-[calc(100vw-64px)]
";
