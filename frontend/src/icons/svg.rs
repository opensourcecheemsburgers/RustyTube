use leptos::*;

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
