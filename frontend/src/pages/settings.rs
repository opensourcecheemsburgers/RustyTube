use std::str::FromStr;

use gloo::{
	file::Blob,
	storage::{LocalStorage, Storage},
};
use invidious::{LocalPlaylist, NewpipeSubscriptions, Subscriptions, SUBS_KEY};
use leptos::{
	component, create_action, expect_context, view, wasm_bindgen, web_sys,
	Children, CollectView, For, IntoView, Props, SignalGet, SignalSet,
	SignalUpdate, StoredValue,
};
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;
use urlencoding::encode;
use utils::get_element_by_id;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlDialogElement, HtmlInputElement, MouseEvent};

use crate::{
	contexts::{RegionConfigCtx, SponsorBlockConfigCtx, UiConfigCtx},
	resources::{
		save_playlists, save_subs, PlaylistsCtx, SubscriptionsCtx,
		SubscriptionsThumbnailsResource, SubscriptionsVideosResource,
	},
	themes::{DARK_THEMES, LIGHT_THEMES},
	utils::i18n,
};

#[component]
pub fn SettingsPage() -> impl IntoView {
	view! {
		<div class="flex flex-col items-center w-full h-full">
			<div class="flex overflow-visible overflow-y-auto overscroll-contain flex-col gap-16 px-6 w-[95vw] my-[3vh] sm:w-[95vw] md:w-[90vw] lg:w-[85vw] xl:w-[50vw]">
				<SubscriptionsSettings/>
				// <PlaylistsSettings/>
				<SponsorBlockSettings/>
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
fn SettingsSection(children: Children, title: String) -> impl IntoView {
	view! {
		<div class="flex flex-col">
			<h1 class="font-sans text-3xl">{title}</h1>
			<div class="divider"></div>
			{children()}
		</div>
	}
}

#[component]
fn Setting(children: Children, title: String) -> impl IntoView {
	view! {
		<div class="w-full form-control">
			<label class="cursor-pointer label">
				<p class="font-mono text-2xl">{title}</p>
				<div class="flex flex-row flex-wrap gap-4 justify-end items-center">
					{children()}
				</div>
			</label>
			<div class="divider"></div>
		</div>
	}
}

#[component]
pub fn SubscriptionsSettings() -> impl IntoView {
	view! {
		<SettingsSection title=i18n("settings.subscriptions")()>
			<Setting title=i18n("settings.manage")()>
				<ImportSubsButton/>
				<DeleteAllSubsButton/>
			</Setting>
			<Setting title=i18n("settings.export")()>
				<ExportSubsFreeTubeButton/>
				<ExportSubsNewPipeButton/>
				<ExportSubsLibreTubeButton/>
			</Setting>
		</SettingsSection>
	}
}

#[component]
pub fn PlaylistsSettings() -> impl IntoView {
	view! {
		<SettingsSection title=i18n("settings.playlists")()>
			<Setting title=i18n("settings.manage")()>
				<ImportPlaylistsButton/>
				<DeleteAllSubsButton/>
			</Setting>
			<Setting title=i18n("settings.export")()>
				<ExportSubsFreeTubeButton/>
				<ExportSubsNewPipeButton/>
				<ExportSubsLibreTubeButton/>
			</Setting>
		</SettingsSection>
	}
}

#[component]
pub fn SponsorBlockSettings() -> impl IntoView {
	let ctx = expect_context::<SponsorBlockConfigCtx>();

	view! {
		<SettingsSection title=i18n("settings.sponsorblock.title")()>
			<Setting title=i18n("settings.sponsorblock.sponsor")()>
				<input
					on:click=move |_| toggle_skip_sponsors(&ctx)
					type="checkbox"
					class="toggle toggle-primary lg:toggle-lg"
					checked=ctx.skip_sponsors.0
				/>
			</Setting>
			<Setting title=i18n("settings.sponsorblock.selfpromo")()>
				<input
					on:input=move |_| toggle_skip_selfpromos(&ctx)
					type="checkbox"
					class="toggle toggle-primary lg:toggle-lg"
					checked=ctx.skip_selfpromos.0
				/>
			</Setting>
			<Setting title=i18n("settings.sponsorblock.intro")()>
				<input
					on:input=move |_| toggle_skip_intros(&ctx)
					type="checkbox"
					class="toggle toggle-primary lg:toggle-lg"
					checked=ctx.skip_intros.0
				/>
			</Setting>
			<Setting title=i18n("settings.sponsorblock.outro")()>
				<input
					on:input=move |_| toggle_skip_outros(&ctx)
					type="checkbox"
					class="toggle toggle-primary lg:toggle-lg"
					checked=move || ctx.skip_outros.0.get()
				/>
			</Setting>
			<Setting title=i18n("settings.sponsorblock.interaction")()>
				<input
					on:input=move |_| toggle_skip_interactions(&ctx)
					type="checkbox"
					class="toggle toggle-primary lg:toggle-lg"
					checked=ctx.skip_interactions.0
				/>
			</Setting>
			<Setting title=i18n("settings.sponsorblock.preview")()>
				<input
					on:input=move |_| toggle_skip_previews(&ctx)
					type="checkbox"
					class="toggle toggle-primary lg:toggle-lg"
					checked=ctx.skip_irrelevant_music.0
				/>
			</Setting>
			<Setting title=i18n("settings.sponsorblock.filler")()>
				<input
					on:input=move |_| toggle_skip_filler(&ctx)
					type="checkbox"
					class="toggle toggle-primary lg:toggle-lg"
					checked=ctx.skip_filler.0
				/>
			</Setting>
		</SettingsSection>
	}
}

fn toggle_enabled(ctx: &SponsorBlockConfigCtx) {
	ctx.enabled.1.set(!ctx.enabled.0.get());
}

fn toggle_skip_sponsors(ctx: &SponsorBlockConfigCtx) {
	ctx.skip_sponsors.1.set(!ctx.skip_sponsors.0.get());
}

fn toggle_skip_selfpromos(ctx: &SponsorBlockConfigCtx) {
	ctx.skip_selfpromos.1.set(!ctx.skip_selfpromos.0.get());
}

fn toggle_skip_interactions(ctx: &SponsorBlockConfigCtx) {
	ctx.skip_interactions.1.set(!ctx.skip_interactions.0.get());
}

fn toggle_skip_intros(ctx: &SponsorBlockConfigCtx) {
	ctx.skip_intros.1.set(!ctx.skip_intros.0.get());
}

fn toggle_skip_outros(ctx: &SponsorBlockConfigCtx) {
	ctx.skip_outros.1.set(!ctx.skip_outros.0.get());
}

fn toggle_skip_previews(ctx: &SponsorBlockConfigCtx) {
	ctx.skip_previews.1.set(!ctx.skip_previews.0.get());
}

fn toggle_skip_irrelevant_music(ctx: &SponsorBlockConfigCtx) {
	ctx.skip_irrelevant_music.1.set(!ctx.skip_irrelevant_music.0.get());
}

fn toggle_skip_filler(ctx: &SponsorBlockConfigCtx) {
	ctx.skip_filler.1.set(!ctx.skip_filler.0.get());
}

#[component]
pub fn RegionSettings() -> impl IntoView {
	view! {
		<SettingsSection title=i18n("settings.locale")()>
			<Setting title=i18n("settings.language")()>
				<LocaleDropdown/>
			</Setting>
			<Setting title=i18n("settings.trending_region")()>
				<TrendingRegionDropdown/>
			</Setting>
		</SettingsSection>
	}
}

#[component]
pub fn LocaleDropdown() -> impl IntoView {
	let locale_slice = expect_context::<RegionConfigCtx>().locale_slice;

	view! {
		<div class="dropdown dropdown-end">
			<div tabindex="0" role="button" class="m-1 btn btn-secondary">
				{move || locale_slice.0.get().human_name()}
			</div>
			<ul
				tabindex="0"
				class="overflow-y-scroll z-10 p-3 w-64 max-h-80 rounded-xl shadow dropdown-content bg-base-300 h-fit"
			>
				<For
					each=move || {
						rust_i18n::available_locales!()
							.into_iter()
							.map(RustyTubeLocale::from_lang_code_str)
							.collect::<Vec<RustyTubeLocale>>()
					}

					key=locales::RustyTubeLocale::id
					let:locale
				>
					<li>
						<a
							class="justify-start text-left btn btn-xs btn-ghost h-fit btn-block md:btn-sm"
							on:click=move |_| locale_slice.1.set(locale)
						>

							<p>{locale.human_name()}</p>
						</a>
					</li>
				</For>
			</ul>
		</div>
	}
}

#[component]
pub fn TrendingRegionDropdown() -> impl IntoView {
	let trending_region_slice =
		expect_context::<RegionConfigCtx>().trending_region_slice;

	let regions_view = isocountry::CountryCode::iter()
		.map(|region| {
			let set_region = move |_| trending_region_slice.1.set(*region);

			view! {
				<li>
					<a
						class="justify-start text-left btn btn-xs btn-ghost h-fit btn-block md:btn-sm"
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
			<div tabindex="0" role="button" class="m-1 btn btn-secondary">
				{move || trending_region_slice.0.get().name()}
			</div>
			<ul
				tabindex="0"
				class="overflow-y-scroll z-10 p-3 w-64 h-80 rounded-xl shadow dropdown-content bg-base-300"
			>

				{regions_view}
			</ul>
		</div>
	}
}

#[component]
pub fn ImportSubsButton() -> impl IntoView {
	let subs = expect_context::<SubscriptionsCtx>();

	let parse_subs_file = create_action(|input: &(SubscriptionsCtx, Event)| {
		let subs = input.0;
		let event = input.1.clone();

		get_subs_from_file(subs, event)
	});

	let on_file_upload = move |event: Event| {
		parse_subs_file.dispatch((subs, event));
	};

	view! {
		<div>
			<label
				class="btn btn-sm btn-primary md:btn-md lg:btn-lg"
				for="subs_upload"
			>
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
	subs_resource: SubscriptionsCtx,
	event: Event,
) -> Result<(), RustyTubeError> {
	let input = event
		.target()
		.ok_or(RustyTubeError::TargetNotFound)?
		.dyn_into::<HtmlInputElement>()
		.ok()
		.ok_or(RustyTubeError::ElementNotFound)?;
	let filelist = input.files().ok_or(RustyTubeError::NoFileSelected)?;
	let file = filelist.get(0).ok_or(RustyTubeError::NoFileSelected)?;
	let blob: Blob = file.into();
	let mut subscriptions = Subscriptions::read_subs(blob).await?;
	subs_resource.0.update(|subs| {
		subs.channels.append(&mut subscriptions.channels);
		subs.channels
			.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
		subs.channels.dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));
		save_subs(subs);
	});
	expect_context::<SubscriptionsVideosResource>().resource.refetch();
	expect_context::<SubscriptionsThumbnailsResource>().resource.refetch();
	Ok(())
}

#[component]
pub fn DeleteAllSubsButton() -> impl IntoView {
	let subs_ctx = expect_context::<SubscriptionsCtx>();

	let modal_id = StoredValue::new("delete_subs_modal");
	let open_modal = move |_| {
		get_element_by_id::<HtmlDialogElement>(modal_id.get_value())
			.expect("donate modal should exist")
			.set_open(true);
	};

	let close_modal = move |_| {
		get_element_by_id::<HtmlDialogElement>(modal_id.get_value())
			.expect("donate modal should exist")
			.set_open(false);
	};

	let delete_all_subs = move |ev: MouseEvent| {
		LocalStorage::set(SUBS_KEY, "");
		subs_ctx.0.set(Subscriptions::default());
		close_modal(ev);
	};

	view! {
		<button
			on:click=open_modal
			class="btn btn-sm btn-error md:btn-md lg:btn-lg"
		>
			{i18n("settings.delete_all")}
		</button>
		<dialog id=modal_id.get_value() class="modal">
			<div class="modal-box">
				<h3 class="text-lg font-bold">
					{i18n("settings.delete_subscriptions")}
				</h3>
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
		format!("data:attachment/text,{encoded_subs}")
	};

	view! {
		<a
			href=href
			download="libretube_subscriptions.json"
			class="btn btn-sm bg-[#000] border-[#000] md:btn-md lg:btn-lg hover:bg-[#000] hover:border-[#000]"
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
		format!("data:attachment/text,{encoded_subs}")
	};

	view! {
		<a
			href=href
			download="freetube_subscriptions.json"
			class="btn btn-sm bg-[#E4E4E4] border-[#E4E4E4] md:btn-md lg:btn-lg hover:bg-[#E4E4E4] hover:border-[#E4E4E4]"
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
		format!("data:attachment/text,{encoded_subs}")
	};

	view! {
		<a
			href=href
			download="newpipe_subscriptions.json"
			class="btn btn-sm bg-[#CD201F] border-[#CD201F] md:btn-md lg:btn-lg hover:bg-[#CD201F] hover:border-[#CD201F]"
		>
			NewPipe
		</a>
	}
}

#[component]
pub fn ImportPlaylistsButton() -> impl IntoView {
	let playlists = expect_context::<PlaylistsCtx>();

	let parse_playlists_file =
		create_action(|input: &(PlaylistsCtx, Event)| {
			let playlists = input.0;
			let event = input.1.clone();

			get_playlists_from_file(playlists, event)
		});

	let on_file_upload = move |event: Event| {
		parse_playlists_file.dispatch((playlists, event));
	};

	view! {
		<div>
			<label
				class="btn btn-sm btn-primary md:btn-md lg:btn-lg"
				for="playlists_upload"
			>
				{i18n("settings.import")}
			</label>
			<input
				id="playlists_upload"
				type="file"
				accept=".ron,.json,.csv"
				multiple=false
				on:change=on_file_upload
				class="hidden"
			/>
		</div>
	}
}

async fn get_playlists_from_file(
	playlists_resource: PlaylistsCtx,
	event: Event,
) -> Result<(), RustyTubeError> {
	let input = event
		.target()
		.expect("playlist button should exist")
		.dyn_into::<HtmlInputElement>()
		.expect("playlist button should be an input element");
	let filelist = input.files().ok_or(RustyTubeError::NoFileSelected)?;
	let file = filelist.get(0).ok_or(RustyTubeError::NoFileSelected)?;
	let blob: Blob = file.into();
	let mut new_playlists = LocalPlaylist::read_playlists(blob).await?;
	playlists_resource.playlists.update(|playlists| {
		playlists.append(&mut new_playlists);
		playlists.sort_by(|a, b| {
			a.title.to_lowercase().cmp(&b.title.to_lowercase())
		});
		playlists.dedup_by(|a, b| a.title.eq_ignore_ascii_case(&b.title));
		save_playlists(playlists);
	});
	Ok(())
}

#[component]
fn ThemeSettings() -> impl IntoView {
	let dark_themes_view = DARK_THEMES
		.iter()
		.map(|theme| view! { <ThemeCard name=(*theme).to_string()/> })
		.collect_view();

	let light_themes_view = LIGHT_THEMES
		.iter()
		.map(|theme| view! { <ThemeCard name=(*theme).to_string()/> })
		.collect_view();

	view! {
		<div class="flex flex-col gap-3 w-full">
			<h1 class="font-sans text-3xl">{i18n("settings.themes")}</h1>
			<div class="divider"></div>
			<div class="flex flex-row flex-wrap gap-4">
				{dark_themes_view} {light_themes_view}
			</div>
		</div>
	}
}

#[component]
pub fn ThemeCard(name: String) -> impl IntoView {
	let theme_name = StoredValue::new(name);

	let current_theme_slice = expect_context::<UiConfigCtx>().theme_slice;

	let card_classes = move || {
		let current_theme = current_theme_slice.0.get();
		if theme_name.get_value().eq_ignore_ascii_case(&current_theme) {
			"lg:w-96 w-64 overflow-hidden rounded-lg border-2 border-primary \
				 hover:border-primary outline-primary outline-8 outline-offset-8"
		} else {
			"lg:w-96 w-64 overflow-hidden rounded-lg border-2 border-base-content/20 \
				 hover:border-base-content/40 outline-base-content outline-2 outline-offset-2"
		}
	};

	let set_theme = move |_| current_theme_slice.1.set(theme_name.get_value());

	view! {
		<div on:click=set_theme class=card_classes>
			<div
				data-theme=theme_name.get_value()
				class="font-sans cursor-pointer bg-base-100 text-base-content"
			>
				<div class="grid grid-cols-5 grid-rows-3">
					<div class="col-start-1 row-span-2 row-start-1 bg-base-200"></div>
					<div class="col-start-1 row-start-3 bg-base-300"></div>
					<div class="flex flex-col col-span-4 col-start-2 row-span-3 row-start-1 gap-1 p-2 bg-base-100">
						<div class="font-bold">{theme_name.get_value()}</div>
						<div class="flex flex-wrap gap-1">
							<div class="flex justify-center items-center w-5 rounded lg:w-6 bg-primary aspect-square">
								<div class="text-sm font-bold text-primary-content">
									{"A"}
								</div>
							</div>
							<div class="flex justify-center items-center w-5 rounded lg:w-6 bg-secondary aspect-square">
								<div class="text-sm font-bold text-secondary-content">
									{"A"}
								</div>
							</div>
							<div class="flex justify-center items-center w-5 rounded lg:w-6 bg-accent aspect-square">
								<div class="text-sm font-bold text-accent-content">
									{"A"}
								</div>
							</div>
							<div class="flex justify-center items-center w-5 rounded lg:w-6 bg-neutral aspect-square">
								<div class="text-sm font-bold text-neutral-content">
									{"A"}
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	}
}
