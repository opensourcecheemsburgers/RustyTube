use std::time::Duration;

use leptos::{
	component, expect_context, view, AnimatedShow, IntoView, Props, SignalSet,
};

use super::control_btns::{
	FormatDropdown, FullScreenBtn, FullWindowBtn, PauseBtn, ProgressBar,
	TimeInfo, VolumeKnob,
};
use crate::contexts::{PlayerStyle, VIDEO_CONTROLS_ID};

#[component]
pub fn VideoPlayerControls() -> impl IntoView {
	let style = expect_context::<PlayerStyle>();
	let show_controls = move |_| style.controls_visible.set(true);

	view! {
		<AnimatedShow
			when=style.controls_visible
			show_class="transition-all opacity-100 duration-1000"
			hide_class="transition-all opacity-0 duration-1000 cursor-none"
			hide_delay=Duration::from_millis(1000)
		>
			<div
				id=VIDEO_CONTROLS_ID
				on:mouseover=show_controls
				class="flex absolute bottom-0 left-0 flex-col w-full group/controls"
			>
				<ProgressBar/>
				<div class="flex flex-row justify-between items-center w-full bg-opacity-50 rounded-b bg-base-300">
					<div class="flex flex-row">
						<PauseBtn/>
						<VolumeKnob/>
						<TimeInfo/>
					</div>
					<div class="flex flex-row">
						<FormatDropdown/>
						// <CaptionsDropdown/>
						<FullWindowBtn/>
						<FullScreenBtn/>
					</div>
				</div>
			</div>
		</AnimatedShow>
	}
}
