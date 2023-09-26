use leptos::*;

use crate::contexts::PlayerState;

#[component]
pub fn TimeInfo(cx: Scope) -> impl IntoView {
    let state = expect_context::<PlayerState>(cx);
    
    view! {cx, 
        <div class="flex flex-row space-x-2 ml-2 font-mono">
            <p>{state.current_time_str}</p>
            <p>/</p>
            <p>{state.duration_str}</p>
        </div>
    }
}
