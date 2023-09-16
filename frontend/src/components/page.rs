use leptos::*;
use crate::components::{Sidebar, Header};

use crate::contexts::ThemeCtx;

#[component]
pub fn Page(cx: Scope, children: Children) -> impl IntoView {
    let theme = expect_context::<ThemeCtx>(cx).0.0;

    let expanded = create_rw_signal(cx, true.to_string());
    provide_context(cx, expanded);

    view! {cx,
        <div data-theme={theme} class="flex flex-row min-h-screen max-h-screen bg-base-100 min-w-screen max-w-screen">
            <Sidebar/>
            <div data-expanded=expanded class=PAGE_CLASSES>
                <Header />
                <div class="min-h-[calc(100vh-4rem)] max-h-[calc(100vh-4rem)] bg-base-100 overflow-hidden">
                    {children(cx)}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ScrollablePage(cx: Scope, children: Children) -> impl IntoView {
    let theme = expect_context::<ThemeCtx>(cx).0.0;

    let expanded = create_rw_signal(cx, true.to_string());
    provide_context(cx, expanded);

    view! {cx,
        <div data-theme={theme} class="flex flex-row min-h-screen max-h-screen bg-base-100">
            <Sidebar/>
            <div data-expanded=expanded class=PAGE_CLASSES>
                <Header />
                <div class="min-h-[calc(100vh-64px)] max-h-[calc(100vh-64px)] bg-base-100 overflow-y-scroll">
                    {children(cx)}
                </div>
            </div>
        </div>
    }
}

pub const PAGE_CLASSES: &'static str = "
flex flex-col

data-[expanded=false]:w-[calc(100vw-16px)]
data-[expanded=true]:w-[calc(100vw-64px)]
";