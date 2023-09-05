use leptos::*;
use crate::components::Sidebar;

use crate::contexts::ThemeCtx;

#[component]
pub fn Page(cx: Scope, children: Children) -> impl IntoView {
    let theme = expect_context::<ThemeCtx>(cx).0.0;

    view! {cx,
        <div data-theme={theme} class="flex flex-row min-h-screen max-h-screen bg-base-100 overflow-hidden">
            <Sidebar/>
            <div class="flex flex-col min-h-screen max-h-screen w-[calc(100vw-4rem)] bg-base-100">
                {children(cx)}
            </div>
        </div>
    }
}