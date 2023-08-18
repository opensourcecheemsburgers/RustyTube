use leptos::*;
use config::Config;

use crate::ThemeCtx;

#[component]
pub fn Page(cx: Scope, children: Children) -> impl IntoView {
    let theme = expect_context::<ThemeCtx>(cx).0.0;

    view! {cx,
        <div data-theme={theme} class="flex flex-col min-h-screen max-h-screen min-w-screen max-w-screen justify-between">
            {children(cx)}
        </div>
    }
}