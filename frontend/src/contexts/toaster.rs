use std::time::Duration;

use leptos::{provide_context, RwSignal, SignalUpdate, StoredValue};

#[derive(Clone, Copy)]
pub struct Toaster {
	pub toasts: RwSignal<Vec<Toast>>,
}

impl Toaster {
	fn new() -> Self {
		Toaster { toasts: RwSignal::new(vec![]) }
	}

	pub fn add_toast(&self, toast: Toast) {
		self.toasts.update(|toasts| toasts.push(toast))
	}

	pub fn remove_toast(&self, burnt_toast: &Toast) {
		self.toasts.update(|toasts| toasts.retain(|toast| toast != burnt_toast))
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Toast {
	pub message: StoredValue<String>,
	pub duration: Option<ToastDuration>,
	pub r#type: Option<ToastType>,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ToastType {
	#[default]
	Normal,
	Info,
	Warning,
	Error,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ToastDuration {
	#[default]
	Normal,
	Long,
}

impl From<ToastDuration> for Duration {
	fn from(value: ToastDuration) -> Duration {
		match value {
			ToastDuration::Normal => Duration::from_millis(1250),
			ToastDuration::Long => Duration::from_millis(2500),
		}
	}
}

pub fn provide_toaster_ctx() {
	provide_context(Toaster::new())
}
