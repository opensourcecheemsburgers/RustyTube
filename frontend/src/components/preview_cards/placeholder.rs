use leptos::*;

#[component]
pub fn PlaceholderCardArray() -> impl IntoView {
    view! {
        <div class="-ml-4 flex flex-row flex-wrap gap-y-12 h-[calc(100vh-13rem)] overflow-y-scroll">
            {(0..50).map(|_| view! { <PlaceholderCard/> }).collect_view()}
        </div>
    }
}

#[component]
pub fn PlaceholderCard() -> impl IntoView {
    view! {
        <div class="basis-1/3 lg:basis-1/4 flex flex-col h-auto px-4 overflow-hidden">
            <div class="animate-pulse w-full aspect-video bg-base-content rounded-xl"></div>
            <div class="flex flex-col w-full mt-3 space-y-3 px-2">
                <div class="animate-pulse w-full h-2 rounded-xl bg-base-content"></div>
                <div class="animate-pulse w-full h-2 rounded-xl bg-base-content"></div>
                <div class="animate-pulse w-[35%] h-2 rounded-xl bg-base-content"></div>
            </div>
        </div>
    }
}

