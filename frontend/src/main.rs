mod components;
mod icons;

use leptos::*;
use components::*;

#[derive(Copy, Clone)]
pub struct ThemeCtx(RwSignal<String>);

#[component]
fn App(cx: Scope) -> impl IntoView {
    let rw_theme = create_rw_signal(cx, String::from("dark"));
    provide_context(cx, ThemeCtx(rw_theme));

    view! {cx,
        <Page>
            <Header />
        </Page>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}