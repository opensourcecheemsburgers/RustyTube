use invidious::{Instance, InstanceInfo, SearchArgs, Suggestions};
use leptos::{html::Input, *};
use phosphor_leptos::{
	ArrowClockwise, ArrowLeft, ArrowRight, ArrowUUpLeft, HardDrives, IconWeight, List, Palette,
};
use rustytube_error::RustyTubeError;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, KeyboardEvent, MouseEvent};

use crate::{
	components::{drawer::DRAWER_ID, FerrisError},
	contexts::{NetworkConfigCtx, RegionConfigCtx, UiConfigCtx},
	resources::{InstancesResource, SearchResource, SearchSuggestions},
	themes::*,
	utils::*,
};

#[component]
pub fn Header() -> impl IntoView {
	view! {
		<div class="navbar bg-base-100 w-full justify-between">
			<div class="flex flex-row items-center justify-start">
				<label
					class="landscape:lg:hidden btn btn-xs sm:btn-sm md:btn-md btn-ghost"
					for=DRAWER_ID
				>
					<List weight=IconWeight::Regular class="h-6 w-6 base-content"/>
				</label>
				<div class="hidden lg:landscape:!flex flex-row items-center">
					<BackBtn/>
					<ForwardBtn/>
					<ReloadBtn/>
				</div>
			</div>
			<div class="flex justify-center flex-row items-center">
				<Search/>
			</div>
			<div class="flex justify-end flex-row items-center">
				<InstanceSelectDropdown/>
				<ThemeSelectDropdown/>
			</div>
		</div>
	}
}

#[component]
pub fn BackBtn() -> impl IntoView {
	view! {
		<div
			class="lg:landscape:tooltip lg:landscape:tooltip-bottom lg:landscape:tooltip-info"
			data-tip=i18n("header.back")
		>
			<button on:click=|_| back().unwrap() class="btn btn-ghost rounded-btn">
				<ArrowLeft weight=IconWeight::Regular class="h-6 w-6 base-content"/>
			</button>
		</div>
	}
}

fn back() -> Result<(), RustyTubeError> {
	Ok(window().history()?.back()?)
}

#[component]
pub fn ForwardBtn() -> impl IntoView {
	view! {
		<div
			class="lg:landscape:lg:landscape:tooltip lg:landscape:tooltip-bottom lg:landscape:tooltip-info"
			data-tip=i18n("header.forward")
		>
			<button on:click=|_| forward().unwrap() class="btn btn-ghost rounded-btn">
				<ArrowRight weight=IconWeight::Regular class="h-6 w-6 base-content"/>
			</button>
		</div>
	}
}

fn forward() -> Result<(), RustyTubeError> {
	Ok(window().history()?.forward()?)
}

#[component]
pub fn ReloadBtn() -> impl IntoView {
	view! {
		<div
			class="lg:landscape:tooltip lg:landscape:tooltip-bottom lg:landscape:tooltip-info"
			data-tip=i18n("header.force_reload")
		>
			<button on:click=|_| reload().unwrap() class="btn btn-ghost rounded-btn">
				<ArrowClockwise weight=IconWeight::Regular class="h-6 w-6 base-content"/>
			</button>
		</div>
	}
}

fn reload() -> Result<(), RustyTubeError> {
	Ok(window().location().reload_with_forceget(true)?)
}

#[component]
pub fn Search() -> impl IntoView {
	let search_bar = create_node_ref::<Input>();

	let query = RwSignal::new(String::new());
	let search_args = RwSignal::new(SearchArgs::from_str("".to_string()));

	let search = move |_| {
		go_to(format!("/search{}", search_args.get().to_url()));
	};

	let check_for_enter_key = move |keyboard_event: KeyboardEvent| {
		if keyboard_event.key_code() == 13 {
			if !query.get().trim().is_empty() {
				go_to(format!("/search{}", search_args.get().to_url()));
			}
		}
	};

	let suggestions = SearchSuggestions::initialise(query);
	let on_input = move |_| {
		if let Some(search_bar) = search_bar.get() {
			query.set(search_bar.value());
			search_args.update(|args| args.query = search_bar.value());
		}
	};

	view! {
		<div class="z-20 dropdown dropdown-bottom dropdown-end">
			<div class="flex">
				<div class="join">
					<input
						on:input=on_input
						on:keydown=check_for_enter_key
						_ref=search_bar
						tabindex="0"
						id="search"
						type="text"
						placeholder=i18n("header.search_placeholder")
						class="input input-sm md:input-md input-bordered input-primary sm:join-item w-48 md:w-60 lg:w-72 xl:w-84 2xl:w-96"
					/>
					<button
						class="hidden sm:!flex btn btn-xs sm:btn-sm md:btn-md btn-primary join-item"
						on:click=search
					>
						{i18n("header.search")}
					</button>
				</div>
				<ul
					tabindex="0"
					class="w-full p-2 rounded-b-lg dropdown-content menu bg-base-200 shadow-dropdown"
				>
					{move || {
						suggestions
							.resource
							.get()
							.map(|suggestions| match suggestions {
								Ok(suggestions) => {
									view! {
										<For
											each=move || suggestions.suggestions.clone()
											key=|suggestion| suggestion.clone()
											let:suggestion
										>
											<li>
												<button on:click={
													let suggestion = suggestion.clone();
													move |_| {
														if let Some(search_bar) = search_bar.get() {
															search_bar.set_value(&suggestion.clone());
														}
														query.set(suggestion.clone());
														search_args.update(|args| args.query = suggestion.clone());
														go_to(format!("/search{}", search_args.get().to_url()));
													}
												}>{suggestion}</button>
											</li>
										</For>
									}
								}
								Err(err) => view! { <FerrisError error=err/> },
							})
					}}

				</ul>
			</div>
		</div>
	}
}

#[component]
pub fn InstanceSelectDropdown() -> impl IntoView {
	let instances = expect_context::<InstancesResource>().resource;

	move || {
		instances.get().map(|instances| {
			view! {
				<div class="dropdown dropdown-end z-50">
					<div
						class="flex flex-row items-center lg:landscape:tooltip lg:landscape:tooltip-bottom lg:landscape:tooltip-info"
						data-tip=i18n("header.instances")
					>

						<label
							tabindex="0"
							class="btn btn-xs sm:btn-sm md:btn-md btn-ghost rounded-btn"
						>
							<HardDrives weight=IconWeight::Regular class="h-6 w-6 base-content"/>

						</label>
					</div>
					<ul
						tabindex="0"
						class="menu dropdown-content px-1.5 py-3 shadow bg-base-300 rounded-xl w-64 h-80 z-10"
					>
						<div class="flex flex-col h-full px-3 space-y-2 overflow-y-scroll">

							{instances
								.unwrap()
								.into_iter()
								.map(|instance: (String, InstanceInfo)| {
									let api = instance.1.api.unwrap_or_default();
									let cors = instance.1.cors.unwrap_or_default();
									let server_visible = api && cors;
									match server_visible {
										true => {
											view! { <InstanceDropdownListItem instance=instance/> }
										}
										false => view! { <div class="hidden"></div> }.into_view(),
									}
								})
								.collect_view()}

						</div>
					</ul>
				</div>
			}
		})
	}
}

#[component]
pub fn InstanceDropdownListItem(instance: Instance) -> impl IntoView {
	let server = expect_context::<NetworkConfigCtx>().server_slice;

	let instance_name = instance.0;
	let flag = instance.1.flag;
	let uri = instance.1.uri;

	let instance_view = move || {
		let instance_name = instance_name.clone();
		let flag = flag.clone();
		let uri = uri.clone();

		match server.0.get().eq_ignore_ascii_case(&uri) {
			false => {
				let uri = uri.clone();

				view! {
					<div
						class="p-3 rounded-lg bg-base-100"
						on:click=move |_| server.1.set(uri.clone())
					>
						<a class="font-sans text-base-content">{flag} {" "} {instance_name}</a>
					</div>
				}
			}
			true => {
				let uri = uri.clone();
				view! {
					<div
						class="p-3 border-2 rounded-lg bg-base-100 border-primary"
						on:click=move |_| server.1.set(uri.clone())
					>
						<a class="font-sans text-base-content">{flag} {" "} {instance_name}</a>
					</div>
				}
			}
		}
	};
	instance_view
}

#[component]
pub fn ThemeDropdownListItem(name: &'static str) -> impl IntoView {
	let theme_ctx = expect_context::<UiConfigCtx>().theme_slice;

	let theme_view = move || match theme_ctx.0.get().eq_ignore_ascii_case(name) {
		true => view! {
			<div
				data-theme=name
				class="p-3 border-2 rounded-lg bg-base-100 border-primary"
				on:click=move |_| theme_ctx.1.set(name.to_string())
			>
				<a class="font-sans capitalize text-base-content">
					<div class="flex flex-row items-center justify-between w-full rounded-lg">
						{name} <div class="flex flex-row gap-1">
							<div data-theme=name class="w-4 h-4 rounded-full bg-primary"></div>
							<div data-theme=name class="w-4 h-4 rounded-full bg-secondary"></div>
							<div data-theme=name class="w-4 h-4 rounded-full bg-accent"></div>
							<div data-theme=name class="w-4 h-4 rounded-full bg-neutral"></div>
						</div>
					</div>
				</a>
			</div>
		},
		false => view! {
			<div
				data-theme=name
				class="p-3 rounded-lg bg-base-100"
				on:click=move |_| theme_ctx.1.set(name.to_string())
			>
				<a class="font-sans capitalize text-base-content">
					<div class="flex flex-row items-center justify-between w-full rounded-lg">
						{name} <div class="flex flex-row gap-1">
							<div data-theme=name class="w-4 h-4 rounded-full bg-primary"></div>
							<div data-theme=name class="w-4 h-4 rounded-full bg-secondary"></div>
							<div data-theme=name class="w-4 h-4 rounded-full bg-accent"></div>
							<div data-theme=name class="w-4 h-4 rounded-full bg-neutral"></div>
						</div>
					</div>
				</a>
			</div>
		},
	};

	theme_view
}

#[component]
pub fn ThemeSelectDropdown() -> impl IntoView {
	let dark_themes_view = DARK_THEMES
		.into_iter()
		.map(|theme| view! { <ThemeDropdownListItem name=theme/> })
		.collect_view();

	let light_themes_view = LIGHT_THEMES
		.into_iter()
		.map(|theme| view! { <ThemeDropdownListItem name=theme/> })
		.collect_view();

	view! {
		<div class="dropdown dropdown-end z-50">
			<div
				class="flex flex-row items-center lg:landscape:tooltip lg:landscape:tooltip-bottom lg:landscape:tooltip-info"
				data-tip=i18n("header.themes")
			>

				<label tabindex="0" class="btn btn-xs sm:btn-sm md:btn-md btn-ghost rounded-btn">
					<Palette weight=IconWeight::Regular class="h-6 w-6 base-content"/>
				</label>
			</div>
			<ul
				tabindex="0"
				class="menu dropdown-content px-1.5 py-3 shadow bg-base-300 rounded-xl w-64 h-80 z-10"
			>
				<div class="flex flex-col h-full px-3 space-y-2 overflow-y-scroll">
					<h1>{i18n("header.dark_themes")}</h1>
					{dark_themes_view}
					<h1>{i18n("header.light_themes")}</h1>
					{light_themes_view}
				</div>
			</ul>
		</div>
	}
}
