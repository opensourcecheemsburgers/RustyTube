use invidious::{fetch_instance_info, Instance, InstanceInfo};
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, InputEvent};

use crate::components::{Tooltip, TooltipPosition};
use crate::contexts::{InstancesCtx, ServerCtx, ThemeCtx};
use crate::icons::{FerrisIcon, PaletteIcon, ServerIcon};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="navbar bg-base-100">
            <div class="navbar-start"></div>
            <div class="navbar-center">
                <div class="form-control">
                    <input
                        type="text"
                        placeholder="Search"
                        class="input input-bordered w-auto md:w-96"
                    />
                </div>
            </div>
            <div class="navbar-end">
                <InstanceSelectDropdown/>
                <ThemeSelectDropdown/>
                <button class="btn btn-square btn-ghost">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        class="inline-block w-5 h-5 stroke-current"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"
                        ></path>
                    </svg>
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn Search() -> impl IntoView {
    let on_search = move |ev: Event| {
        let search_bar: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
        let search_str = search_bar.value();
    };

    view! {
        <div class="form-control">
            <input
                on:input=on_search
                id="search"
                type="text"
                placeholder="Search"
                class="input input-bordered w-auto md:w-96"
            />
        </div>
    }
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
                            <div class="flex flex-col h-full overflow-y-scroll space-y-2 px-3">

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
                        <a class="text-base-content font-sans">{flag} {" "} {instance_name}</a>
                    </div>
                }
            }
            true => {
                let uri = uri.clone();
                view! {
                    <div
                        class="p-3 rounded-lg bg-base-100 border-2 border-primary"
                        on:click=move |_| server.1.set(uri.clone())
                    >
                        <a class="text-base-content font-sans">{flag} {" "} {instance_name}</a>
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
                class="p-3 rounded-lg bg-base-100 border-2 border-primary"
                on:click=move |_| theme_ctx.1.set(name.to_string())
            >
                <a class="capitalize text-base-content font-sans">
                    <div class="flex flex-row justify-between w-full items-center rounded-lg">
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
                <a class="capitalize text-base-content font-sans">
                    <div class="flex flex-row justify-between w-full items-center rounded-lg">
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
                <div class="flex flex-col h-full overflow-y-scroll space-y-2 px-3">

                    {THEMES
                        .into_iter()
                        .map(|theme| view! { <ThemeDropdownListItem name=theme/> })
                        .collect_view()}

                </div>
            </ul>
        </div>
    }
}

pub const THEMES: &'static [&'static str] = &[
    "dracula",
    "winter",
    "night",
    "synthwave",
    "aqua",
    "retro",
    "cyberpunk",
    "valentine",
    "halloween",
    "light",
    "garden",
    "forest",
    "dark",
    "black",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "luxury",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "coffee",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
];

