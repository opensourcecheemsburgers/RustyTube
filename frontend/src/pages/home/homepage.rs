use crate::components::{Header, Page};
use crate::pages::home::trending::TrendingSection;
use leptos::*;
use config::HomepageCategory::*;
use rustytube_error::RustyTubeError;
use crate::contexts::HomepageCategoryCtx;
use crate::pages::home::popular::PopularSection;
use crate::pages::home::subscriptions::SubscriptionsSection;

#[component]
pub fn Homepage() -> impl IntoView {
    let category = expect_context::<HomepageCategoryCtx>().0.0;

    let current = move || match category.get() {
        Trending => view! {<TrendingSection />},
        Subscriptions => view! {<SubscriptionsSection />},
        Popular => view! {<PopularSection />}
    };

    view!{
        <Page>
            { current }
        </Page>
    }
}

#[component]
pub fn HomepageSection(children: Children) -> impl IntoView {
    view!{
        <div class="w-full flex justify-center mt-4">
            <div class="w-[90%] flex flex-col gap-y-8">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn HomepageSectionTitle(title: String) -> impl IntoView {
    view!{
        <h1 class="pl-4 font-semibold text-2xl">{title}</h1>
    }
}
