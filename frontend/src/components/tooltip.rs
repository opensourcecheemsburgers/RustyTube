use leptos::*;

#[derive(PartialEq, Clone, Copy)]
pub enum TooltipPosition {
	Top,
	Bottom,
	Left,
	Right,
}

impl Default for TooltipPosition {
	fn default() -> Self {
		TooltipPosition::Bottom
	}
}

#[component]
pub fn Tooltip(children: Children, tip: &'static str, position: TooltipPosition) -> impl IntoView {
	let mut tooltip_classes = String::from("overflow-visible tooltip tooltip-info block ");

	match position {
		TooltipPosition::Top => tooltip_classes.push_str("tooltip-top"),
		TooltipPosition::Bottom => tooltip_classes.push_str("tooltip-bottom"),
		TooltipPosition::Left => tooltip_classes.push_str("tooltip-left"),
		TooltipPosition::Right => tooltip_classes.push_str("tooltip-right"),
	}

	view! {
		<div data-tip=tip class=tooltip_classes>
			{children()}
		</div>
	}
}
