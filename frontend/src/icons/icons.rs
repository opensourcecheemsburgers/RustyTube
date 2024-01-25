use std::ops::Div;

use leptos::*;

use super::svg::Svg;

pub const ICON: &'static str = "h-4 w-4";
pub const LARGE_ICON: &'static str = "h-6 w-6";

#[component]
pub fn HamburgerIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<line x1="4" x2="20" y1="12" y2="12"></line>
			<line x1="4" x2="20" y1="6" y2="6"></line>
			<line x1="4" x2="20" y1="18" y2="18"></line>
		</Svg>
	}
}

#[component]
pub fn FerrisIcon() -> impl IntoView {
	view! { <img class="w-8" src="ferris/cute.svg"/> }
}

#[component]
pub fn FerrisWaveIcon(width: u8) -> impl IntoView {
	let width_style = format!(r#"width: {}rem"#, width.div(4));

	view! { <img style=width_style src="ferris/wave.svg"/> }
}

#[component]
pub fn FerrisWtfIcon(width: u8) -> impl IntoView {
	let width_style = format!(r#"width: {}rem"#, width.div(4));

	view! { <img style=width_style src="ferris/wtf.svg"/> }
}

#[component]
pub fn PauseIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<rect width="4" height="16" x="6" y="4"></rect>
			<rect width="4" height="16" x="14" y="4"></rect>
		</Svg>
	}
}

#[component]
pub fn PlayIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<polygon points="5 3 19 12 5 21 5 3"></polygon>
		</Svg>
	}
}

#[component]
pub fn VolumeDefaultIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
		</Svg>
	}
}

#[component]
pub fn VolumeMediumIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
			<path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
		</Svg>
	}
}

#[component]
pub fn VolumeHighIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
			<path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
			<path d="M19.07 4.93a10 10 0 0 1 0 14.14"></path>
		</Svg>
	}
}

#[component]
pub fn FullScreenIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<path d="M8 3H5a2 2 0 0 0-2 2v3"></path>
			<path d="M21 8V5a2 2 0 0 0-2-2h-3"></path>
			<path d="M3 16v3a2 2 0 0 0 2 2h3"></path>
			<path d="M16 21h3a2 2 0 0 0 2-2v-3"></path>
		</Svg>
	}
}

#[component]
pub fn FullWindowIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<polyline points="15 3 21 3 21 9"></polyline>
			<polyline points="9 21 3 21 3 15"></polyline>
			<line x1="21" x2="14" y1="3" y2="10"></line>
			<line x1="3" x2="10" y1="21" y2="14"></line>
		</Svg>
	}
}

#[component]
pub fn CaptionsIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<path d="M7 13h4"></path>
			<path d="M15 13h2"></path>
			<path d="M7 9h2"></path>
			<path d="M13 9h4"></path>
			<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10Z"></path>
		</Svg>
	}
}

#[component]
pub fn CogIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<path d="M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z"></path>
			<path d="M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"></path>
			<path d="M12 2v2"></path>
			<path d="M12 22v-2"></path>
			<path d="m17 20.66-1-1.73"></path>
			<path d="M11 10.27 7 3.34"></path>
			<path d="m20.66 17-1.73-1"></path>
			<path d="m3.34 7 1.73 1"></path>
			<path d="M14 12h8"></path>
			<path d="M2 12h2"></path>
			<path d="m20.66 7-1.73 1"></path>
			<path d="m3.34 17 1.73-1"></path>
			<path d="m17 3.34-1 1.73"></path>
			<path d="m11 13.73-4 6.93"></path>
		</Svg>
	}
}

#[component]
pub fn PaletteIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<circle cx="13.5" cy="6.5" r=".5"></circle>
			<circle cx="17.5" cy="10.5" r=".5"></circle>
			<circle cx="8.5" cy="7.5" r=".5"></circle>
			<circle cx="6.5" cy="12.5" r=".5"></circle>
			<path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z"></path>
		</Svg>
	}
}

#[component]
pub fn ServerIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<rect width="20" height="8" x="2" y="2" rx="2" ry="2"></rect>
			<rect width="20" height="8" x="2" y="14" rx="2" ry="2"></rect>
			<line x1="6" x2="6.01" y1="6" y2="6"></line>
			<line x1="6" x2="6.01" y1="18" y2="18"></line>
		</Svg>
	}
}

#[component]
pub fn SubscriptionsIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<path d="M4 11a9 9 0 0 1 9 9"></path>
			<path d="M4 4a16 16 0 0 1 16 16"></path>
			<circle cx="5" cy="19" r="1"></circle>
		</Svg>
	}
}

#[component]
pub fn TrendingIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<polyline points="22 7 13.5 15.5 8.5 10.5 2 17"></polyline>
			<polyline points="16 7 22 7 22 13"></polyline>
		</Svg>
	}
}

#[component]
pub fn PopularIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"></path>
		</Svg>
	}
}

#[component]
pub fn SettingsIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"></path>
			<circle cx="12" cy="12" r="3"></circle>
		</Svg>
	}
}

#[component]
pub fn HeartIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>
		</Svg>
	}
}

#[component]
pub fn ViewsIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"></path>
			<circle cx="12" cy="12" r="3"></circle>
		</Svg>
	}
}

#[component]
pub fn LikeIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<path d="M7 10v12"></path>
			<path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z"></path>
		</Svg>
	}
}

#[component]
pub fn DislikeIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<path d="M17 14V2"></path>
			<path d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z"></path>
		</Svg>
	}
}

#[component]
pub fn CalendarIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<rect width="18" height="18" x="3" y="4" rx="2" ry="2"></rect>
			<line x1="16" x2="16" y1="2" y2="6"></line>
			<line x1="8" x2="8" y1="2" y2="6"></line>
			<line x1="3" x2="21" y1="10" y2="10"></line>
			<path d="M8 14h.01"></path>
			<path d="M12 14h.01"></path>
			<path d="M16 14h.01"></path>
			<path d="M8 18h.01"></path>
			<path d="M12 18h.01"></path>
			<path d="M16 18h.01"></path>
		</Svg>
	}
}

#[component]
pub fn RepliesIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
		</Svg>
	}
}

#[component]
pub fn DownloadIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
			<polyline points="7 10 12 15 17 10"></polyline>
			<line x1="12" x2="12" y1="15" y2="3"></line>
		</Svg>
	}
}

#[component]
pub fn ShareIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<circle cx="18" cy="5" r="3"></circle>
			<circle cx="6" cy="12" r="3"></circle>
			<circle cx="18" cy="19" r="3"></circle>
			<line x1="8.59" x2="15.42" y1="13.51" y2="17.49"></line>
			<line x1="15.41" x2="8.59" y1="6.51" y2="10.49"></line>
		</Svg>
	}
}

#[component]
pub fn BackIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<path d="m12 19-7-7 7-7"></path>
			<path d="M19 12H5"></path>
		</Svg>
	}
}

#[component]
pub fn ForwardIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<path d="M5 12h14"></path>
			<path d="m12 5 7 7-7 7"></path>
		</Svg>
	}
}

#[component]
pub fn ReloadIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"></path>
			<path d="M3 3v5h5"></path>
			<path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"></path>
			<path d="M16 16h5v5"></path>
		</Svg>
	}
}

#[component]
pub fn PlaylistAddIcon() -> impl IntoView {
	view! {
		<Svg class=LARGE_ICON>
			<path d="M11 12H3"></path>
			<path d="M16 6H3"></path>
			<path d="M16 18H3"></path>
			<path d="M18 9v6"></path>
			<path d="M21 12h-6"></path>
		</Svg>
	}
}

#[component]
pub fn VerifiedIcon() -> impl IntoView {
	view! {
		<Svg class=ICON>
			<circle cx="12" cy="12" r="10"></circle>
			<path d="m9 12 2 2 4-4"></path>
		</Svg>
	}
}
