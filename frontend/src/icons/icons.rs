use std::ops::Div;

use leptos::{component, view, IntoView, Props};

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
