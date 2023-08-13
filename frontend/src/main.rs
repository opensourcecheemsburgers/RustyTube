use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="text-4xl font-display font-semibold">
            Welcome to RustyTube!
        </div>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}