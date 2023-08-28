use crate::components::{FerrisError, Header, Page};
use crate::pages::home::trending::TrendingSection;
use leptos::*;
use config::HomepageCategory::*;
use rustytube_error::RustyTubeError;
use crate::contexts::UiConfigCtx;
use crate::pages::home::popular::PopularSection;
use crate::pages::home::subscriptions::SubscriptionsSection;

#[component]
pub fn Homepage(cx: Scope) -> impl IntoView {
    let category = expect_context::<UiConfigCtx>(cx).0.0.get().homepage;

    let current = move || match category {
        Trending => view! {cx, <TrendingSection />},
        Subscriptions => view! {cx, <SubscriptionsSection />},
        Popular => view! {cx, <PopularSection />}
    };

    view! {cx,
        <Page>
            <Header />
            { current }
        </Page>

    }
}

#[component]
pub fn HomepageSection(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="w-screen flex justify-center mt-4">
            <div class="w-[90%] flex flex-col gap-y-8">
                {children(cx)}
            </div>
        </div>
    }
}

#[component]
pub fn HomepageSectionTitle(cx: Scope, title: String) -> impl IntoView {
    view! {cx,
        <h1 class="font-semibold text-2xl">{title}</h1>
    }
}
