mod components;
mod icons;

use leptos::*;
use components::*;

use console_error_panic_hook;
use config::Config;

#[derive(Copy, Clone)]
pub struct ThemeCtx(RwSignal<String>);

#[component]
fn App(cx: Scope) -> impl IntoView {
    console_error_panic_hook::set_once();

    let config = create_rw_signal(cx, Config::default());
    let rw_theme = create_rw_signal(cx, String::from("dark"));
    provide_context(cx, ThemeCtx(rw_theme));
    provide_context(cx, config);

    view! {cx,
        <Page>
            <Header />
        </Page>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}