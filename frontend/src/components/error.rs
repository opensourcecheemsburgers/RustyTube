use leptos::{component, view, IntoView, Props};
use rustytube_error::RustyTubeError;

#[allow(clippy::needless_pass_by_value)]
#[component]
pub fn FerrisError(error: RustyTubeError) -> impl IntoView {
	view! {
		<div class="flex flex-col items-center p-2 space-y-8 max-w-xl h-auto bg-inherit w-[70%] text-base-content">
			<img class="w-[80%]" src="ferris/wtf.svg"/>
			<p class="font-mono text-base font-normal w-fit">
				{error.to_string()}
			</p>
		</div>
	}
}
