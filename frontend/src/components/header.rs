use config::Config;
use leptos::*;
use web_sys::MouseEvent;
use crate::ThemeCtx;
// use crate::ThemeCtx;
use crate::icons::{HamburgerIcon, FerrisIcon};
use crate::components::{Tooltip, TooltipPosition};
use crate::icons::PaletteIcon;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="navbar bg-base-100">
            <div class="navbar-start">
                <label class="btn btn-ghost">
                    <HamburgerIcon />
                </label>
                <div class="lg:flex lg:flex-1 lg:ml-4 hidden">
                    <a class="btn btn-ghost normal-case text-2xl font-display font-bold tracking-wide">
                        <FerrisIcon />
                        <p class="ml-2">Rusty</p>
                        <p>Tube</p>
                    </a>
                </div>
            </div>
            <div class="navbar-center">
                <div class="form-control">
                    <input type="text" placeholder="Search" class="input input-bordered w-auto md:w-96" />
                </div>
            </div>
            <div class="navbar-end">
                <ThemeSelectDropdown />
                <button class="btn btn-square btn-ghost">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"></path></svg>
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn ThemeDropdownListItem(
    cx: Scope,
    name: &'static str,
) -> impl IntoView {
    let theme_ctx = expect_context::<ThemeCtx>(cx).0.write_only();
    // let switch_theme = move |_| {
    //     gloo::console::debug!("Switching theme to {}",  name);
    //     WriteSignal::set(&theme_ctx, name.to_string());
    // };
    
    view! {cx,
        <div data-theme={name} class="p-3 rounded-lg bg-base-100" on:click=move |_| {
                gloo::console::debug!("Switching theme to {}",  name);
                WriteSignal::set(&theme_ctx, name.to_string());
            }>
            <a class="capitalize text-base-content font-sans">
                <div class="flex flex-row justify-between w-full items-center rounded-lg">
                    {name}
                    <div class="flex flex-row gap-1">
                        <div data-theme={name} class="w-4 h-4 rounded-full bg-primary"></div>
                        <div data-theme={name} class="w-4 h-4 rounded-full bg-secondary"></div>
                        <div data-theme={name} class="w-4 h-4 rounded-full bg-accent"></div>
                        <div data-theme={name} class="w-4 h-4 rounded-full bg-neutral"></div>
                    </div>
                </div>
            </a>
        </div>
    }
}

#[component]
pub fn ThemeSelectDropdown(cx: Scope) -> impl IntoView {    
    view! {cx,
        <div class="dropdown dropdown-end">
            <Tooltip tip={"Themes"} position={TooltipPosition::Bottom}>
                <label tabindex="0" class="btn btn-ghost rounded-btn">
                    <PaletteIcon />
                </label>
            </Tooltip>
            <ul tabindex="0" class="menu dropdown-content px-1.5 py-3 shadow bg-base-300 rounded-xl w-64 h-80">
                <div class="flex flex-col h-full overflow-y-scroll space-y-2 px-3">
                    {
                        THEMES.into_iter().map(|theme| view!
                            { cx,
                                <ThemeDropdownListItem name={theme} />
                            }
                        ).collect_view(cx)
                    }
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