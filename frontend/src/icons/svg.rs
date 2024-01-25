use leptos::*;

pub const RESPONSIVE_ICON_LG: &'static str = "h-5 w-5 lg:h-6 lg:w-6 2xl:h-7 2xl:w-7";
pub const RESPONSIVE_ICON: &'static str = "h-4 w-4 lg:h-5 lg:w-5 2xl:h-6 2xl:w-6";

#[component]
pub fn Svg(children: Children, class: &'static str) -> impl IntoView {
	/// UI Icons from https://lucide.dev

	const STROKE: &'static str = "currentColor";
	const STROKE_WIDTH: &'static str = "1.5";
	const BOX: &'static str = "0 0 24 24";
	const FILL: &'static str = "none";
	const LCAP: &'static str = "round";
	const LJOIN: &'static str = "round";

	view! {
		<svg
			class=class
			viewBox=BOX
			fill=FILL
			stroke=STROKE
			stroke-width=STROKE_WIDTH
			stroke-linecap=LCAP
			stroke-linejoin=LJOIN
		>
			{children()}
		</svg>
	}
}
