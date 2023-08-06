use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            Welcome to RustyTube!
        </div>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}