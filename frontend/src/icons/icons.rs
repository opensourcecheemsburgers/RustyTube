use std::ops::Div;

use leptos::*;

use super::svg::Svg;

pub const ICON: &'static str = "h-4 w-4";
pub const LARGE_ICON: &'static str = "h-6 w-6";

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
