use invidious::{DashFormat, Format, Formats};
use leptos::{
	component, expect_context, view, CollectView, IntoView, Props, RwSignal,
	SignalGet,
};
use phosphor_leptos::{GearFine, IconWeight};

use crate::{contexts::PlayerState, pages::video::utils::find_audio_format};

#[component]
pub fn FormatDropdown() -> impl IntoView {
	view! {
		<div class="z-20 dropdown dropdown-top dropdown-end">
			<DropdownBtn/>
			<DropdownContent/>
		</div>
	}
}

#[component]
pub fn DropdownBtn() -> impl IntoView {
	view! {
		<label tabindex="0" class="btn btn-ghost btn-xs lg:btn-sm">
			<GearFine
				weight=IconWeight::Regular
				class="w-4 h-4 lg:w-5 lg:h-5 base-content"
			/>
		</label>
	}
}

#[component]
pub fn DropdownContent() -> impl IntoView {
	view! {
		<ul
			tabindex="0"
			class="py-3 px-1.5 mb-4 w-max rounded-xl shadow menu dropdown-content bg-base-200 h-max"
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
				let quality_str = format.audio_quality.to_string();

				let change_format = move |_| {
					let _ = state.change_format(Format::Audio(format.clone()));
				};

				view! {
					<button
						on:click=change_format
						class="lowercase btn btn-xs btn-ghost md:btn-sm"
					>
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
						format!("{} - ({})", format.quality_label, container)
					},
				);

				let audio_format = current_format.get().map_or(
					find_audio_format(&formats.get()).ok(),
					|current_format| current_format.audio_format(),
				);

				let change_format = move |_| {
					state.change_format(Format::Dash(DashFormat::new(
						format.clone(),
						audio_format
							.clone()
							.expect("Adaptive format must have audio."),
					)));
				};

				view! {
					<button
						on:click=change_format
						class="lowercase btn btn-xs btn-ghost md:btn-sm"
					>
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
				let quality_str = format.quality_label.to_string();

				let change_format = move |_| {
					let _ = state.change_format(Format::Legacy(format.clone()));
				};

				view! {
					<button
						on:click=change_format
						class="lowercase btn btn-xs btn-ghost md:btn-sm"
					>
						{quality_str}
					</button>
				}
			})
			.collect_view()
	};

	view! {
		<div class="flex flex-row gap-x-4 p-2 w-max rounded-lg h-max bg-base-200">
			<div class="flex flex-col items-center">
				<h1>Audio</h1>
				<div class="flex overflow-y-scroll flex-col my-4 h-48 lg:h-64">
					{audio_formats_view}
				</div>
			</div>

			<div class="flex flex-col items-center">
				<h1>Legacy</h1>
				<div class="flex overflow-y-scroll flex-col my-4 h-48 lg:h-64">
					{legacy_formats_view}
				</div>
			</div>

			<div class="flex flex-col items-center">
				<h1>Dash</h1>
				<div class="flex overflow-y-scroll flex-col my-4 h-48 lg:h-64">
					{adaptive_formats_view}
				</div>
			</div>
		</div>
	}
}
