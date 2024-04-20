use std::borrow::Cow;

use gloo::{
	file::Blob,
	storage::{LocalStorage, Storage},
};
use invidious::{NewpipeSubscriptions, Subscriptions, SUBS_KEY};
use isocountry::CountryCode;
use leptos::*;
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;
use urlencoding::encode;
use utils::get_element_by_id;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, MouseEvent};

use crate::{
	contexts::{RegionConfigCtx, SubscriptionsCtx, UiConfigCtx},
	themes::*,
	utils::i18n,
};

#[component]
pub fn SettingsPage() -> impl IntoView {
	view! {
		<div class="flex flex-col w-full h-full items-center">
			<div class="flex flex-col 2xl:w-[50vw] xl:w-[50vw] lg:w-[85vw] md:w-[90vw] sm:w-[95vw] my-[3vh] px-6 overscroll-contain overflow-visible overflow-y-auto gap-16">
				<DataSettings/>
				<RegionSettings/>
				<ThemeSettings/>
			</div>
		</div>
	}
}

#[component]
pub fn InstanceSettings() -> impl IntoView {
	view! {}
}

#[component]
fn SettingsSection(children: Children, title: Cow<'static, str>) -> impl IntoView {
	view! {
		<div class="flex flex-col">
			<h1 class="font-sans text-3xl">{title}</h1>
			<div class="divider"></div>
			{children()}
			<div class="divider"></div>
		</div>
	}
}

#[component]
fn Setting(children: Children, title: Cow<'static, str>) -> impl IntoView {
	view! {
		<div class="form-control w-full">
			<label class="cursor-pointer label">
				<p class="font-mono text-2xl">{title}</p>
				{children()}
			</label>
			<div class="divider"></div>
		</div>
	}
}

#[component]
pub fn RegionSettings() -> impl IntoView {
	view! {
		<div class="flex flex-col">
			<h1 class="font-sans text-3xl">{i18n("settings.locale")}</h1>
			<div class="divider"></div>
			<div class="form-control w-full">
				<label class="cursor-pointer label">
					<p class="font-mono text-2xl">{i18n("settings.language")}</p>
					<div class="flex flex-row justify-end gap-4">
						<LocaleDropdown/>
					</div>
				</label>
				<div class="divider"></div>
			</div>
			<div class="form-control w-full">
				<label class="cursor-pointer label">
					<p class="font-mono text-2xl">{i18n("settings.trending_region")}</p>
					<div class="flex flex-row justify-end gap-4">
						<TrendingRegionDropdown/>
					</div>
				</label>
				<div class="divider"></div>
			</div>
		</div>
	}
}

#[component]
pub fn LocaleDropdown() -> impl IntoView {
	let locale_slice = expect_context::<RegionConfigCtx>().locale_slice;

	let locales_view = rust_i18n::available_locales!()
		.into_iter()
		.map(|available_locale| {
			let locale = RustyTubeLocale::from_str(available_locale);
			let set_locale = move |_| locale_slice.1.set(locale);

			view! {
				<li>
					<a
						class="btn btn-sm btn-ghost h-fit btn-block justify-start text-left"
						on:click=set_locale
					>
						<p>{locale.human_name()}</p>
					</a>
				</li>
			}
		})
		.collect_view();

	view! {
		<div class="dropdown dropdown-end">
			<div tabindex="0" role="button" class="btn btn-secondary m-1">
				{move || locale_slice.0.get().human_name()}
			</div>
			<ul
				tabindex="0"
				class="overflow-y-scroll dropdown-content p-3 shadow bg-base-300 rounded-xl w-64 max-h-80 h-fit z-10"
			>
				{locales_view}
			</ul>
		</div>
	}
}

#[component]
pub fn TrendingRegionDropdown() -> impl IntoView {
	let trending_region_slice = expect_context::<RegionConfigCtx>().trending_region_slice;

	let regions_view = isocountry::CountryCode::iter()
		.map(|region| {
			let set_region = move |_| trending_region_slice.1.set(region.clone());

			view! {
				<li>
					<a
						class="btn btn-sm btn-ghost h-fit btn-block justify-start text-left"
						on:click=set_region
					>
						<p>{region.name()}</p>
					</a>
				</li>
			}
		})
		.collect_view();

	view! {
		<div class="dropdown dropdown-end">
			<div tabindex="0" role="button" class="btn btn-secondary m-1">
				{move || trending_region_slice.0.get().name()}
			</div>
			<ul
				tabindex="0"
				class="overflow-y-scroll dropdown-content p-3 shadow bg-base-300 rounded-xl w-64 h-80 z-10"
			>

				{regions_view}
			</ul>
		</div>
	}
}

#[component]
pub fn DataSettings() -> impl IntoView {
	view! {
		<div class="flex flex-col">
			<h1 class="font-sans text-3xl">{i18n("settings.subscriptions")}</h1>
			<div class="divider"></div>
			<div class="form-control w-full">
				<label class="cursor-pointer label">
					<p class="font-mono text-2xl">{i18n("settings.manage")}</p>
					<div class="flex flex-row justify-end gap-4">
						<ImportSubsButton/>
						<DeleteAllSubsButton/>
					</div>
				</label>
				<div class="divider"></div>
			</div>
			<div class="form-control w-full">
				<label class="cursor-pointer label">
					<p class="font-mono text-2xl">{i18n("settings.export")}</p>
					<div class="flex flex-row justify-end gap-4">
						<ExportSubsFreeTubeButton/>
						<ExportSubsNewPipeButton/>
						<ExportSubsLibreTubeButton/>
					</div>
				</label>
				<div class="divider"></div>
			</div>
		</div>
	}
}

#[component]
pub fn ImportSubsButton() -> impl IntoView {
	let subs = expect_context::<SubscriptionsCtx>().0;

	let parse_subs_file = create_action(|input: &(RwSignal<Subscriptions>, Event)| {
		let subs = input.0.clone();
		let event = input.1.clone();

		get_subs_from_file(subs, event)
	});

	let on_file_upload = move |event: Event| {
		parse_subs_file.dispatch((subs, event));
	};

	view! {
		<div>
			<label class="btn btn-lg btn-primary" for="subs_upload">
				{i18n("settings.import")}
			</label>
			<input
				id="subs_upload"
				type="file"
				accept=".ron,.json,.csv"
				multiple=false
				on:change=on_file_upload
				class="hidden"
			/>
		</div>
	}
}

async fn get_subs_from_file(
	subs: RwSignal<Subscriptions>,
	event: Event,
) -> Result<(), RustyTubeError> {
	let input = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
	let filelist = input.files().ok_or(RustyTubeError::no_file_selected())?;
	let file = filelist.get(0).ok_or(RustyTubeError::no_file_selected())?;
	let blob: Blob = file.into();
	let mut subscriptions = Subscriptions::read_subs(blob).await?;
	subscriptions.save().await?;
	subs.update(|existing_subscriptions| {
		existing_subscriptions.channels.append(&mut subscriptions.channels);
		existing_subscriptions
			.channels
			.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
		existing_subscriptions.channels.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name))
	});
	Ok(())
}

#[component]
pub fn DeleteAllSubsButton() -> impl IntoView {
	let subs_ctx = expect_context::<SubscriptionsCtx>();

	let modal_id = StoredValue::new("delete_subs_modal");
	let open_modal = move |_| {
		get_element_by_id::<HtmlDialogElement>(modal_id.get_value()).unwrap().set_open(true);
	};

	let close_modal = move |_| {
		get_element_by_id::<HtmlDialogElement>(modal_id.get_value()).unwrap().set_open(false);
	};

	let delete_all_subs = move |ev: MouseEvent| {
		LocalStorage::set(SUBS_KEY, "").unwrap();
		subs_ctx.0.update(|subs| subs.channels.clear());
		close_modal(ev);
	};

	view! {
		<button on:click=open_modal class="btn btn-lg btn-error">
			{i18n("settings.delete_all")}
		</button>
		<dialog id=modal_id class="modal">
			<div class="modal-box">
				<h3 class="font-bold text-lg">{i18n("settings.delete_subscriptions")}</h3>
				<p class="py-4">
					This action will delete all subscriptions from the RustyTube database.
				</p>
				<div class="modal-action">
					<button on:click=close_modal class="btn btn-ghost">
						{i18n("settings.close")}
					</button>
					<button on:click=delete_all_subs class="btn btn-error">
						{i18n("settings.delete_all")}
					</button>
				</div>
			</div>
		</dialog>
	}
}

#[component]
pub fn ExportSubsLibreTubeButton() -> impl IntoView {
	let current_subs = expect_context::<SubscriptionsCtx>().0;

	let href = move || {
		let subs: NewpipeSubscriptions = current_subs.get().into();
		let subs_json = subs.to_json_string().unwrap_or_default();
		let encoded_subs = encode(&subs_json);
		format!("data:attachment/text,{}", encoded_subs)
	};

	view! {
		<a
			href=href
			download="libretube_subscriptions.json"
			class="btn btn-lg bg-[#000] border-[#000] hover:bg-[#000] hover:border-[#000]"
		>
			<div class="flex flex-row">
				<p class="text-[#FF9698]">Libre</p>
				<p class="text-white">Tube</p>
			</div>
		</a>
	}
}

#[component]
pub fn ExportSubsFreeTubeButton() -> impl IntoView {
	let current_subs = expect_context::<SubscriptionsCtx>().0;

	let href = move || {
		let subs: NewpipeSubscriptions = current_subs.get().into();
		let subs_json = subs.to_json_string().unwrap_or_default();
		let encoded_subs = encode(&subs_json);
		format!("data:attachment/text,{}", encoded_subs)
	};

	view! {
		<a
			href=href
			download="freetube_subscriptions.json"
			class="btn btn-lg  bg-[#E4E4E4] border-[#E4E4E4] hover:bg-[#E4E4E4] hover:border-[#E4E4E4]"
		>
			<div class="flex flex-row">
				<p class="text-[#F04242]">Free</p>
				<p class="text-[#29ABE1]">Tube</p>
			</div>
		</a>
	}
}

#[component]
pub fn ExportSubsNewPipeButton() -> impl IntoView {
	let current_subs = expect_context::<SubscriptionsCtx>().0;

	let href = move || {
		let subs: NewpipeSubscriptions = current_subs.get().into();
		let subs_json = subs.to_json_string().unwrap_or_default();
		let encoded_subs = encode(&subs_json);
		format!("data:attachment/text,{}", encoded_subs)
	};

	view! {
		<a
			href=href
			download="newpipe_subscriptions.json"
			class="btn btn-lg bg-[#CD201F] border-[#CD201F] hover:bg-[#CD201F] hover:border-[#CD201F]"
		>
			NewPipe
		</a>
	}
}

#[component]
fn ThemeSettings() -> impl IntoView {
	let dark_themes_view = DARK_THEMES
		.into_iter()
		.map(|theme| view! { <ThemeCard name=theme.to_string()/> })
		.collect_view();

	let light_themes_view = LIGHT_THEMES
		.into_iter()
		.map(|theme| view! { <ThemeCard name=theme.to_string()/> })
		.collect_view();

	view! {
		<div class="flex flex-col gap-3 w-full">
			<h1 class="font-sans text-3xl">{i18n("settings.themes")}</h1>
			<div class="divider"></div>
			<div class="flex flex-wrap flex-row gap-4">{dark_themes_view} {light_themes_view}</div>
		</div>
	}
}

#[component]
pub fn ThemeCard(name: String) -> impl IntoView {
	let theme_name = StoredValue::new(name);

	let current_theme_slice = expect_context::<UiConfigCtx>().theme_slice;

	let card_classes = move || {
		let current_theme = current_theme_slice.0.get();
		match theme_name.get_value().eq_ignore_ascii_case(&current_theme) {
			false => {
				"lg:w-96 w-64 overflow-hidden rounded-lg border-2 border-base-content/20 \
				 hover:border-base-content/40 outline-base-content outline-2 outline-offset-2"
			}
			true => {
				"lg:w-96 w-64 overflow-hidden rounded-lg border-2 border-primary \
				 hover:border-primary outline-primary outline-8 outline-offset-8"
			}
		}
	};

	let set_theme = move |_| current_theme_slice.1.set(theme_name.get_value());

	view! {
		<div on:click=set_theme class=card_classes>
			<div
				data-theme=theme_name
				class="bg-base-100 text-base-content cursor-pointer font-sans"
			>
				<div class="grid grid-cols-5 grid-rows-3">
					<div class="bg-base-200 col-start-1 row-span-2 row-start-1"></div>
					<div class="bg-base-300 col-start-1 row-start-3"></div>
					<div class="bg-base-100 col-span-4 col-start-2 row-span-3 row-start-1 flex flex-col gap-1 p-2">
						<div class="font-bold">{theme_name}</div>
						<div class="flex flex-wrap gap-1">
							<div class="bg-primary flex aspect-square w-5 items-center justify-center rounded lg:w-6">
								<div class="text-primary-content text-sm font-bold">{"A"}</div>
							</div>
							<div class="bg-secondary flex aspect-square w-5 items-center justify-center rounded lg:w-6">
								<div class="text-secondary-content text-sm font-bold">{"A"}</div>
							</div>
							<div class="bg-accent flex aspect-square w-5 items-center justify-center rounded lg:w-6">
								<div class="text-accent-content text-sm font-bold">{"A"}</div>
							</div>
							<div class="bg-neutral flex aspect-square w-5 items-center justify-center rounded lg:w-6">
								<div class="text-neutral-content text-sm font-bold">{"A"}</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	}
}
