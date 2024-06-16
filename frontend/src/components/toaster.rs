use leptos::{
	component, expect_context, set_timeout, view, For, IntoView, Props,
	SignalGet,
};

use crate::contexts::{Toast, ToastType, Toaster};

#[component]
pub fn Toaster<V, DV, IV, WV, EV>(
	default_view: DV,
	info_view: IV,
	warning_view: WV,
	error_view: EV,
) -> impl IntoView
where
	V: IntoView + 'static,
	DV: Fn(Toast) -> V + 'static + Copy,
	IV: Fn(Toast) -> V + 'static + Copy,
	WV: Fn(Toast) -> V + 'static + Copy,
	EV: Fn(Toast) -> V + 'static + Copy,
{
	let toaster = expect_context::<Toaster>();

	view! {
		<div class="toast toast-center z-[100]">
			<For each=move || toaster.toasts.get() key=|toast| *toast let:toast>

				{
					set_timeout(
						move || toaster.remove_toast(&toast),
						toast.duration.unwrap_or_default().into(),
					);
					match toast.r#type.unwrap_or_default() {
						ToastType::Normal => default_view(toast),
						ToastType::Info => info_view(toast),
						ToastType::Warning => warning_view(toast),
						ToastType::Error => error_view(toast),
					}
				}

			</For>
		</div>
	}
}
