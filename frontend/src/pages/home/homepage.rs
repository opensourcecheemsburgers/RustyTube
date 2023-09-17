use crate::components::{Header, Page};
use crate::pages::home::trending::TrendingSection;
use leptos::*;
use config::HomepageCategory::*;
use rustytube_error::RustyTubeError;
use crate::contexts::HomepageCategoryCtx;
use crate::pages::home::popular::PopularSection;
use crate::pages::home::subscriptions::SubscriptionsSection;

#[component]
pub fn Homepage(cx: Scope) -> impl IntoView {
    let category = expect_context::<HomepageCategoryCtx>(cx).0.0;

    let current = move || match category.get() {
        Trending => view! {cx, <TrendingSection />},
        Subscriptions => view! {cx, <SubscriptionsSection />},
        Popular => view! {cx, <PopularSection />}
    };

    view! {cx,
        <Page>
            { current }
        </Page>
    }
}

#[component]
pub fn HomepageSection(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="w-full flex justify-center mt-4">
            <div class="w-[90%] flex flex-col gap-y-8">
                {children(cx)}
            </div>
        </div>
    }
}

#[component]
pub fn HomepageSectionTitle(cx: Scope, title: String) -> impl IntoView {
    view! {cx,
        <h1 class="pl-4 font-semibold text-2xl">{title}</h1>
    }
}
