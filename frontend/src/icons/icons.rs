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
            <img class="w-8" src="ferris/cute.svg" />
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

#[component]
pub fn SubscriptionsIcon(cx: Scope) -> impl IntoView {
    view! {cx,
        <Svg height={24} width={24}>
            <path d="M4 11a9 9 0 0 1 9 9"/>
            <path d="M4 4a16 16 0 0 1 16 16"/>
            <circle cx="5" cy="19" r="1"/>
        </Svg>
    }
}

#[component]
pub fn TrendingIcon(cx: Scope) -> impl IntoView {
    view! {cx,
        <Svg height={24} width={24}>
            <polyline points="22 7 13.5 15.5 8.5 10.5 2 17"/>
            <polyline points="16 7 22 7 22 13"/>
        </Svg>
    }
}

#[component]
pub fn PopularIcon(cx: Scope) -> impl IntoView {
    view! {cx,
        <Svg height={24} width={24}>
            <path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"/>
        </Svg>
    }
}

#[component]
pub fn SettingsIcon(cx: Scope) -> impl IntoView {
    view! {cx,
        <Svg height={24} width={24}>
            <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
            <circle cx="12" cy="12" r="3"/>
        </Svg>
    }
}


#[component]
pub fn HeartIcon(cx: Scope) -> impl IntoView {
    view! {cx,
        <Svg height={24} width={24}>
            <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"/>
        </Svg>
    }
}