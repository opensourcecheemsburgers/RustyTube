#![feature(let_chains)]

mod components;
mod icons;
mod pages;

use leptos::*;
use leptos_router::*;
use components::*;

use console_error_panic_hook;
use config::Config;

use crate::pages::Trending;

#[derive(Copy, Clone)]
pub struct ThemeCtx((Signal<String>, SignalSetter<String>));

#[derive(Copy, Clone)]
pub struct ServerCtx((Signal<String>, SignalSetter<String>));

#[component]
fn App(cx: Scope) -> impl IntoView {
    console_error_panic_hook::set_once();

    let config = create_rw_signal(cx, Config::default());

    let server_ctx: ServerCtx = ServerCtx(create_slice(
        cx,
        config,
        |config| config.network.server.clone(),
        |config, server| {
            config.network.server = server;
            config.save().ok();
        }
    ));

    let theme_ctx: ThemeCtx = ThemeCtx(create_slice(
        cx,
        config,
        |config| config.ui.theme.clone(),
        |config, theme| {
            config.ui.theme = theme;
            config.save().ok();
        }
    ));

    provide_context(cx, theme_ctx);
    provide_context(cx, server_ctx);

    view! {cx,
        <Page>
            <Header />
            <Trending />
        </Page>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}