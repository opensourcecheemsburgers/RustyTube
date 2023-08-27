use std::ops::Div;

use leptos::{component, IntoView, Scope, view};
use super::svg::Svg;

#[component]
pub fn HamburgerIcon(cx: Scope) -> impl IntoView {
	view! {cx,
            <Svg height={24} width={24}>
                <line x1="4" x2="20" y1="12" y2="12"/>
                <line x1="4" x2="20" y1="6" y2="6"/>
                <line x1="4" x2="20" y1="18" y2="18"/>
            </Svg>
        }
}

#[component]
pub fn FerrisIcon(cx: Scope) -> impl IntoView {
	view! {cx,
            <img class="h-10 w-10" src="ferris/cute.svg" />
        }
}

#[component]
pub fn FerrisWaveIcon(cx: Scope, width: u8) -> impl IntoView {
    let width_style = format!(r#"width: {}rem"#, width.div(4));

	view! {cx,
            <img style=width_style src="ferris/wave.svg" />
        }
}

#[component]
pub fn FerrisWtfIcon(cx: Scope, width: u8) -> impl IntoView {
	let width_style = format!(r#"width: {}rem"#, width.div(4));

	view! {cx,
            <img style=width_style src="ferris/wtf.svg" />
        }
}

#[component]
pub fn PauseIcon(cx: Scope) -> impl IntoView {
	view! {cx,
            <Svg height={16} width={16}>
                <rect width="4" height="16" x="6" y="4"/>
                <rect width="4" height="16" x="14" y="4"/>
            </Svg>
        }
}

#[component]
pub fn PlayIcon(cx: Scope) -> impl IntoView {
	view! {cx,
            <Svg height={16} width={16}>
                <polygon points="5 3 19 12 5 21 5 3"/>
            </Svg>
        }
}

#[component]
pub fn VolumeDefaultIcon(cx: Scope) -> impl IntoView {
	view! {cx,
            <Svg height={16} width={16}>
                <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/>
            </Svg>
        }
}

#[component]
pub fn VolumeMediumIcon(cx: Scope) -> impl IntoView {
	view! {cx,
            <Svg height={16} width={16}>
                <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/>
                <path d="M15.54 8.46a5 5 0 0 1 0 7.07"/>
            </Svg>
        }
}

#[component]
pub fn VolumeHighIcon(cx: Scope) -> impl IntoView {
	view! {cx,
            <Svg height={16} width={16}>
                <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/>
                <path d="M15.54 8.46a5 5 0 0 1 0 7.07"/>
                <path d="M19.07 4.93a10 10 0 0 1 0 14.14"/>
            </Svg>
        }
}

#[component]
pub fn FullScreenIcon(cx: Scope) -> impl IntoView {
	view! {cx,
        <Svg height={16} width={16}>
            <path d="M8 3H5a2 2 0 0 0-2 2v3"/>
            <path d="M21 8V5a2 2 0 0 0-2-2h-3"/>
            <path d="M3 16v3a2 2 0 0 0 2 2h3"/>
            <path d="M16 21h3a2 2 0 0 0 2-2v-3"/>
        </Svg>
        }
}

#[component]
pub fn FullWindowIcon(cx: Scope) -> impl IntoView {
	view! {cx,
            <Svg height={16} width={16}>
                <polyline points="15 3 21 3 21 9"/>
                <polyline points="9 21 3 21 3 15"/>
                <line x1="21" x2="14" y1="3" y2="10"/>
                <line x1="3" x2="10" y1="21" y2="14"/>
            </Svg>
        }
}

#[component]
pub fn CaptionsIcon(cx: Scope) -> impl IntoView {
	view! {cx,
        <Svg height={16} width={16}>
            <path d="M7 13h4"/>
            <path d="M15 13h2"/>
            <path d="M7 9h2"/>
            <path d="M13 9h4"/>
            <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10Z"/>
        </Svg>
        }
}

#[component]
pub fn CogIcon(cx: Scope) -> impl IntoView {
	view! {cx,
        <Svg height={16} width={16}>
            <path d="M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z"/>
            <path d="M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"/>
            <path d="M12 2v2"/>
            <path d="M12 22v-2"/>
            <path d="m17 20.66-1-1.73"/>
            <path d="M11 10.27 7 3.34"/>
            <path d="m20.66 17-1.73-1"/>
            <path d="m3.34 7 1.73 1"/>
            <path d="M14 12h8"/>
            <path d="M2 12h2"/>
            <path d="m20.66 7-1.73 1"/>
            <path d="m3.34 17 1.73-1"/>
            <path d="m17 3.34-1 1.73"/>
            <path d="m11 13.73-4 6.93"/>
        </Svg>
        }
}

#[component]
pub fn PaletteIcon(cx: Scope) -> impl IntoView {
	view! {cx,
            <Svg height={24} width={24}>
                <circle cx="13.5" cy="6.5" r=".5"/>
                <circle cx="17.5" cy="10.5" r=".5"/>
                <circle cx="8.5" cy="7.5" r=".5"/>
                <circle cx="6.5" cy="12.5" r=".5"/>
                <path
                    d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z">
                </path>
            </Svg>
        }
}

#[component]
pub fn ServerIcon(cx: Scope) -> impl IntoView {
	view! {cx,
            <Svg height={24} width={24}>
                <rect width="20" height="8" x="2" y="2" rx="2" ry="2"/>
                <rect width="20" height="8" x="2" y="14" rx="2" ry="2"/>
                <line x1="6" x2="6.01" y1="6" y2="6"/>
                <line x1="6" x2="6.01" y1="18" y2="18"/>
            </Svg>
        }
}