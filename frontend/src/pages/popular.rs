use invidious::Popular;
use leptos::{component, view, For, IntoView, Props, SignalGet, Suspense};

use crate::{
	components::{
		CardGrid, FerrisError, GridContainer, PlaceholderCardArray,
		PopularPreviewCard,
	},
	resources::PopularResource,
	utils::i18n,
};

#[component]
pub fn PopularSection() -> impl IntoView {
	let popular = PopularResource::initialise();

	view! {
		<GridContainer>
			<h1 class="text-2xl font-semibold">{i18n("sidebar.popular")}</h1>
			<Suspense fallback=move || {
				view! { <PlaceholderCardArray/> }
			}>
				{move || {
					popular
						.resource
						.get()
						.map(|popular_videos_res| {
							match popular_videos_res {
								Ok(popular) => {
									view! {
										<CardGrid>
											<For
												each=move || popular.items.clone()
												key=|video| video.id.clone()
												let:video
											>
												<PopularPreviewCard video=video/>
											</For>
										</CardGrid>
									}
								}
								Err(err) => view! { <FerrisError error=err/> },
							}
						})
				}}

			</Suspense>
		</GridContainer>
	}
}
