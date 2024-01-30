use invidious::{Instance, InstanceInfo, SearchArgs, Suggestions};
use leptos::{html::Input, window, *};
use rustytube_error::RustyTubeError;
use web_sys::KeyboardEvent;

use crate::{
	components::{FerrisError, Tooltip, TooltipPosition},
	contexts::{InstancesCtx, ServerCtx, ThemeCtx},
	icons::{BackIcon, ForwardIcon, PaletteIcon, ReloadIcon, ServerIcon},
	themes::*,
};

#[component]
pub fn Header() -> impl IntoView {
	view! {
		<div class="navbar bg-base-100">
			<div class="navbar-start">
				<BackBtn/>
				<ForwardBtn/>
				<ReloadBtn/>
			</div>
			<div class="navbar-center">
				<Search/>
			</div>
			<div class="navbar-end">
				<InstanceSelectDropdown/>
				<ThemeSelectDropdown/>
			</div>
		</div>
	}
}

#[component]
pub fn BackBtn() -> impl IntoView {
	view! {
		<Tooltip tip="Back" position=TooltipPosition::Bottom>
			<button on:click=|_| back().unwrap() class="btn btn-ghost rounded-btn">
				<BackIcon/>
			</button>
		</Tooltip>
	}
}

fn back() -> Result<(), RustyTubeError> {
	Ok(window().history()?.back()?)
}

#[component]
pub fn ForwardBtn() -> impl IntoView {
	view! {
		<Tooltip tip="Forward" position=TooltipPosition::Bottom>
			<button on:click=|_| forward().unwrap() class="btn btn-ghost rounded-btn">
				<ForwardIcon/>
			</button>
		</Tooltip>
	}
}

fn forward() -> Result<(), RustyTubeError> {
	Ok(window().history()?.forward()?)
}

#[component]
pub fn ReloadBtn() -> impl IntoView {
	view! {
		<Tooltip tip="Force reload" position=TooltipPosition::Bottom>
			<button on:click=|_| reload().unwrap() class="btn btn-ghost rounded-btn">
				<ReloadIcon/>
			</button>
		</Tooltip>
	}
}

fn reload() -> Result<(), RustyTubeError> {
	Ok(window().location().reload_with_forceget(true)?)
}

#[component]
pub fn Search() -> impl IntoView {
	let search_bar = create_node_ref::<Input>();

	let on_search = move |_| {
		let query = search_bar.get().unwrap().value();
		if !query.is_empty() {
			search(query);
		}
	};

	let check_for_enter_key = move |keyboard_event: KeyboardEvent| {
		if keyboard_event.key_code() == 13 {
			let query = search_bar.get().unwrap().value();
			if !query.is_empty() {
				search(query);
			}
		}
	};

	let server = expect_context::<ServerCtx>().0 .0;
	let query = RwSignal::new(String::default());
	let set_query = move |_| query.set(search_bar.get().unwrap().value());

	let suggestions = create_resource(
		move || (query.get(), server.get()),
		|(query, server)| async move { Suggestions::fetch_suggestions(&query, &server).await },
	);

	let suggestions_view = move || {
		suggestions.get().map(|suggestions| match suggestions {
			Ok(suggestions) => suggestions
				.suggestions
				.into_iter()
				.map(|suggestion| {
					let query = suggestion.clone();
					let on_click = move |_| search(query.clone());
					view! {
						<li>
							<button on:click=on_click>{suggestion}</button>
						</li>
					}
				})
				.collect_view(),
			Err(err) => view! { <FerrisError error=err/> },
		})
	};

	view! {
		<div class="z-20 dropdown dropdown-bottom dropdown-end">
			<div class="join">
				<input
					on:input=set_query
					on:keydown=check_for_enter_key
					_ref=search_bar
					tabindex="0"
					id="search"
					type="text"
					placeholder="Type your search here..."
					class="input input-bordered input-primary w-96 join-item "
				/>
				<button class="btn btn-primary join-item" on:click=on_search>
					Search
				</button>
			</div>
			<ul
				tabindex="0"
				class="w-full p-2 rounded-b-lg dropdown-content menu bg-base-200 shadow-dropdown"
			>
				{suggestions_view}
			</ul>
		</div>
	}
}

fn search(query: String) {
	let search_args = SearchArgs::from_str(query);

	let navigate = leptos_router::use_navigate();
	request_animation_frame(move || {
		_ = navigate(&format!("/search{}", search_args.to_url()), Default::default());
	})
}

#[component]
pub fn InstanceSelectDropdown() -> impl IntoView {
	let instances = expect_context::<InstancesCtx>().0;

	view! {
		{move || match instances.get() {
			None => view! { <div></div> },
			Some(instances) => {
				view! {
					<div class="dropdown dropdown-end">
						<Tooltip tip="Instances" position=TooltipPosition::Bottom>
							<label tabindex="0" class="btn btn-ghost rounded-btn">
								<ServerIcon/>
							</label>
						</Tooltip>
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
			}
		}}
	}
}

#[component]
pub fn InstanceDropdownListItem(instance: Instance) -> impl IntoView {
	let server = expect_context::<ServerCtx>().0;

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
	let theme_ctx = expect_context::<ThemeCtx>().0;

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
		<div class="dropdown dropdown-end">
			<Tooltip tip="Themes" position=TooltipPosition::Bottom>
				<label tabindex="0" class="btn btn-ghost rounded-btn">
					<PaletteIcon/>
				</label>
			</Tooltip>
			<ul
				tabindex="0"
				class="menu dropdown-content px-1.5 py-3 shadow bg-base-300 rounded-xl w-64 h-80 z-10"
			>
				<div class="flex flex-col h-full px-3 space-y-2 overflow-y-scroll">
					<h1>Dark Themes</h1>
					{dark_themes_view}
					<h1>Light Themes</h1>
					{light_themes_view}
				</div>
			</ul>
		</div>
	}
}
