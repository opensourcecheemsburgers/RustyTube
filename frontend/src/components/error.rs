use leptos::*;
use rustytube_error::RustyTubeError;
use crate::icons::FerrisWtfIcon;

#[component]
pub fn FerrisError(cx: Scope, error: RustyTubeError, width: u8) -> impl IntoView {
    view! {cx,
        <div class="bg-inherit w-full h-auto flex flex-col space-y-8 items-center p-2 text-base-content">
            <FerrisWtfIcon width=width />
            <h1 class=" w-fit font-semibold text-xl">{error.title}</h1>
            <p class="w-fit font-normal text-base font-mono">{error.description}</p>
        </div>
    }
}