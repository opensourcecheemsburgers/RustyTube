use invidious::{Instance, InstanceInfo, SearchArgs};
use leptos::{
	component, create_node_ref, expect_context, html, view, window,
	CollectView, For, IntoView, RwSignal, SignalGet, SignalSet, SignalUpdate,
};
use phosphor_leptos::{
	ArrowClockwise, ArrowLeft, ArrowRight, HardDrives, IconWeight, List,
	Palette,
};
use rustytube_error::RustyTubeError;
use web_sys::KeyboardEvent;

use crate::{
	components::{drawer::DRAWER_ID, FerrisError},
	contexts::{NetworkConfigCtx, UiConfigCtx},
	resources::{InstancesResource, SearchSuggestions},
	themes::{DARK_THEMES, LIGHT_THEMES},
	utils::{go_to, i18n},
};

#[component]
pub fn Header() -> impl IntoView {
	view! {
		<div class="justify-between w-full navbar bg-base-100">
			<div class="flex flex-row justify-start items-center">
				<label
					class="landscape:lg:hidden btn btn-xs btn-ghost sm:btn-sm md:btn-md"
					for=DRAWER_ID
				>
					<List
						weight=IconWeight::Regular
						class="w-6 h-6 base-content"
					/>
				</label>
				<div class="hidden flex-row items-center lg:landscape:!flex">
					<BackBtn/>
					<ForwardBtn/>
					<ReloadBtn/>
				</div>
			</div>
			<div class="flex flex-row justify-center items-center">
				<Search/>
			</div>
			<div class="flex flex-row justify-end items-center">
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
			<button on:click=|_| back() class="btn btn-ghost rounded-btn">
				<ArrowLeft
					weight=IconWeight::Regular
					class="w-6 h-6 base-content"
				/>
			</button>
		</div>
	}
}

fn back() {
	window()
		.history()
		.expect("Window should have history.")
		.back()
		.expect("Window should be able to go back");
}

#[component]
pub fn ForwardBtn() -> impl IntoView {
	view! {
		<div
			class="lg:landscape:lg:landscape:tooltip lg:landscape:tooltip-bottom lg:landscape:tooltip-info"
			data-tip=i18n("header.forward")
		>
			<button on:click=|_| forward() class="btn btn-ghost rounded-btn">
				<ArrowRight
					weight=IconWeight::Regular
					class="w-6 h-6 base-content"
				/>
			</button>
		</div>
	}
}

fn forward() {
	window()
		.history()
		.expect("Window should have history.")
		.forward()
		.expect("Window should be able to go forward");
}

#[component]
pub fn ReloadBtn() -> impl IntoView {
	view! {
		<div
			class="lg:landscape:tooltip lg:landscape:tooltip-bottom lg:landscape:tooltip-info"
			data-tip=i18n("header.force_reload")
		>
			<button on:click=|_| reload() class="btn btn-ghost rounded-btn">
				<ArrowClockwise
					weight=IconWeight::Regular
					class="w-6 h-6 base-content"
				/>
			</button>
		</div>
	}
}

fn reload() {
	window()
		.location()
		.reload_with_forceget(true)
		.expect("Window should be able to go reload");
}

#[component]
pub fn Search() -> impl IntoView {
	let search_bar = create_node_ref::<html::Input>();

	let query = RwSignal::new(String::new());
	let search_args = RwSignal::new(SearchArgs::from_query_str(String::new()));

	let search = move |_| {
		go_to(format!("/search{}", search_args.get().to_url()));
	};

	let check_for_enter_key = move |keyboard_event: KeyboardEvent| {
		if keyboard_event.key_code() == 13 && !query.get().trim().is_empty() {
			go_to(format!("/search{}", search_args.get().to_url()));
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
						class="w-48 md:w-60 lg:w-72 2xl:w-96 input input-sm input-bordered input-primary sm:join-item md:input-md xl:w-84"
					/>
					<button
						class="hidden btn btn-xs btn-primary join-item sm:!flex sm:btn-sm md:btn-md"
						on:click=search
					>
						{i18n("header.search")}
					</button>
				</div>
				<ul
					tabindex="0"
					class="p-2 w-full rounded-b-lg dropdown-content menu bg-base-200 shadow-dropdown"
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
											key=std::clone::Clone::clone
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
				<div class="z-50 dropdown dropdown-end">
					<div
						class="flex flex-row items-center lg:landscape:tooltip lg:landscape:tooltip-bottom lg:landscape:tooltip-info"
						data-tip=i18n("header.instances")
					>

						<label
							tabindex="0"
							class="btn btn-xs btn-ghost rounded-btn sm:btn-sm md:btn-md"
						>
							<HardDrives
								weight=IconWeight::Regular
								class="w-6 h-6 base-content"
							/>

						</label>
					</div>
					<ul
						tabindex="0"
						class="z-10 py-3 px-1.5 w-64 h-80 rounded-xl shadow menu dropdown-content bg-base-300"
					>
						<div class="flex overflow-y-scroll flex-col px-3 space-y-2 h-full">

							{instances
								.map(|instances| {
									instances
										.into_iter()
										.map(|instance: (String, InstanceInfo)| {
											let api = instance.1.api.unwrap_or_default();
											let cors = instance.1.cors.unwrap_or_default();
											let server_visible = api && cors;
											if server_visible {
												view! { <InstanceDropdownListItem instance=instance/> }
											} else {
												view! { <div class="hidden"></div> }.into_view()
											}
										})
										.collect_view()
								})}

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

	move || {
		let instance_name = instance_name.clone();
		let flag = flag.clone();
		let uri = uri.clone();

		if server.0.get().eq_ignore_ascii_case(&uri) {
			view! {
				<div
					class="p-3 rounded-lg border-2 bg-base-100 border-primary"
					on:click=move |_| server.1.set(uri.clone())
				>
					<a class="font-sans text-base-content">
						{flag} {" "} {instance_name}
					</a>
				</div>
			}
		} else {
			view! {
				<div
					class="p-3 rounded-lg bg-base-100"
					on:click=move |_| server.1.set(uri.clone())
				>
					<a class="font-sans text-base-content">
						{flag} {" "} {instance_name}
					</a>
				</div>
			}
		}
	}
}

#[component]
pub fn ThemeDropdownListItem(name: &'static str) -> impl IntoView {
	let theme_ctx = expect_context::<UiConfigCtx>().theme_slice;

	move || {
		if theme_ctx.0.get().eq_ignore_ascii_case(name) {
			view! {
				<div
					data-theme=name
					class="p-3 rounded-lg border-2 bg-base-100 border-primary"
					on:click=move |_| theme_ctx.1.set(name.to_string())
				>
					<a class="font-sans capitalize text-base-content">
						<div class="flex flex-row justify-between items-center w-full rounded-lg">
							{name} <div class="flex flex-row gap-1">
								<div
									data-theme=name
									class="w-4 h-4 rounded-full bg-primary"
								></div>
								<div
									data-theme=name
									class="w-4 h-4 rounded-full bg-secondary"
								></div>
								<div
									data-theme=name
									class="w-4 h-4 rounded-full bg-accent"
								></div>
								<div
									data-theme=name
									class="w-4 h-4 rounded-full bg-neutral"
								></div>
							</div>
						</div>
					</a>
				</div>
			}
		} else {
			view! {
				<div
					data-theme=name
					class="p-3 rounded-lg bg-base-100"
					on:click=move |_| theme_ctx.1.set(name.to_string())
				>
					<a class="font-sans capitalize text-base-content">
						<div class="flex flex-row justify-between items-center w-full rounded-lg">
							{name} <div class="flex flex-row gap-1">
								<div
									data-theme=name
									class="w-4 h-4 rounded-full bg-primary"
								></div>
								<div
									data-theme=name
									class="w-4 h-4 rounded-full bg-secondary"
								></div>
								<div
									data-theme=name
									class="w-4 h-4 rounded-full bg-accent"
								></div>
								<div
									data-theme=name
									class="w-4 h-4 rounded-full bg-neutral"
								></div>
							</div>
						</div>
					</a>
				</div>
			}
		}
	}
}

#[component]
pub fn ThemeSelectDropdown() -> impl IntoView {
	let dark_themes_view = DARK_THEMES
		.iter()
		.map(|theme| view! { <ThemeDropdownListItem name=theme/> })
		.collect_view();

	let light_themes_view = LIGHT_THEMES
		.iter()
		.map(|theme| view! { <ThemeDropdownListItem name=theme/> })
		.collect_view();

	view! {
		<div class="z-50 dropdown dropdown-end">
			<div
				class="flex flex-row items-center lg:landscape:tooltip lg:landscape:tooltip-bottom lg:landscape:tooltip-info"
				data-tip=i18n("header.themes")
			>

				<label
					tabindex="0"
					class="btn btn-xs btn-ghost rounded-btn sm:btn-sm md:btn-md"
				>
					<Palette
						weight=IconWeight::Regular
						class="w-6 h-6 base-content"
					/>
				</label>
			</div>
			<ul
				tabindex="0"
				class="z-10 py-3 px-1.5 w-64 h-80 rounded-xl shadow menu dropdown-content bg-base-300"
			>
				<div class="flex overflow-y-scroll flex-col px-3 space-y-2 h-full">
					<h1>{i18n("header.dark_themes")}</h1>
					{dark_themes_view}
					<h1>{i18n("header.light_themes")}</h1>
					{light_themes_view}
				</div>
			</ul>
		</div>
	}
}
