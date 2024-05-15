use leptos::*;
use rustytube_error::RustyTubeError;

#[component]
pub fn FerrisError(error: RustyTubeError) -> impl IntoView {
	view! {
		<div class="bg-inherit w-[70%] max-w-xl h-auto flex flex-col space-y-8 items-center p-2 text-base-content">
			<img class="w-[80%]" src="ferris/wtf.svg"/>
			<p class="w-fit font-normal text-base font-mono">{error.to_string()}</p>
		</div>
	}
}
