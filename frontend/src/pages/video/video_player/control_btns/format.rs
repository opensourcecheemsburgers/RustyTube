use invidious::{DashFormat, Format, Formats};
use leptos::*;

use crate::{contexts::PlayerState, icons::CogIcon, pages::video::utils::find_audio_format};

#[component]
pub fn FormatDropdown() -> impl IntoView {
	view! {
		<div class="dropdown dropdown-top dropdown-end z-20">
			<DropdownBtn/>
			<DropdownContent/>
		</div>
	}
}

#[component]
pub fn DropdownBtn() -> impl IntoView {
	view! {
		<label tabindex="0" class="btn btn-ghost btn-xs">
			<CogIcon/>
		</label>
	}
}

#[component]
pub fn DropdownContent() -> impl IntoView {
	view! {
		<ul
			tabindex="0"
			class="menu dropdown-content mb-4 px-1.5 py-3 shadow bg-base-200 rounded-xl w-max h-max"
		>
			<FormatList/>
		</ul>
	}
}

#[component]
pub fn FormatList() -> impl IntoView {
	let current_format = expect_context::<RwSignal<Option<Format>>>();
	let formats = expect_context::<RwSignal<Formats>>();
	let state = expect_context::<PlayerState>();

	let audio_formats_view = move || {
		formats
			.get()
			.audio_formats
			.into_iter()
			.map(|format| {
				let quality_str = format.audio_quality.clone().to_string();

				let change_format = move |_| {
					let _ = state.change_format(Format::Audio(format.clone()));
				};

				view! {
					<button on:click=change_format class="btn btn-sm lowercase btn-ghost">
						{quality_str}
					</button>
				}
			})
			.collect_view()
	};

	let adaptive_formats_view = move || {
		formats
			.get()
			.video_formats
			.into_iter()
			.map(|format| {
				let info_str = format.clone().container.map_or(
					format.quality_label.to_string(),
					|container| {
						format!(
							"{} - ({})",
							format.quality_label.to_string(),
							container.to_string()
						)
					},
				);

				let audio_format = current_format
					.get()
					.map_or(find_audio_format(&formats.get()).ok(), |current_format| {
						current_format.audio_format()
					});

				let change_format = move |_| {
					let _ = state.change_format(Format::Dash(DashFormat::new(
						format.clone(),
						audio_format.clone().unwrap(),
					)));
				};

				view! {
					<button on:click=change_format class="btn btn-sm lowercase btn-ghost">
						{info_str}
					</button>
				}
			})
			.collect_view()
	};

	let legacy_formats_view = move || {
		formats
			.get()
			.legacy_formats
			.into_iter()
			.map(|format| {
				let quality_str = format.quality_label.clone().to_string();

				let change_format = move |_| {
					let _ = state.change_format(Format::Legacy(format.clone()));
				};

				view! {
					<button on:click=change_format class="btn btn-sm lowercase btn-ghost">
						{quality_str}
					</button>
				}
			})
			.collect_view()
	};

	view! {
		<div class="flex h-max w-max flex-row space-x-4 rounded-lg bg-base-200 p-2">
			<div class="flex flex-col items-center">
				<h1>Audio</h1>
				<div class="my-4 flex flex-col h-64 overflow-y-scroll">{audio_formats_view}</div>
			</div>

			<div class="flex flex-col items-center">
				<h1>Legacy</h1>
				<div class="my-4 flex flex-col h-64 overflow-y-scroll">{legacy_formats_view}</div>
			</div>

			<div class="flex flex-col items-center">
				<h1>Dash</h1>
				<div class="my-4 flex flex-col h-64 overflow-y-scroll">{adaptive_formats_view}</div>
			</div>
		</div>
	}
}
