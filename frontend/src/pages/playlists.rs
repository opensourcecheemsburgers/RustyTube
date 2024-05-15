use invidious::{Trending, TrendingCategory};
use leptos::*;

use crate::{
	components::{
		CardGrid, FerrisError, GridContainer, LocalPlaylistPreviewCard, PlaceholderCardArray,
		PlaylistPreviewCard, VideoPreviewCard,
	},
	contexts::{NetworkConfigCtx, RegionConfigCtx},
	resources::{PlaylistsCtx, TrendingResource},
	utils::i18n,
};

#[component]
pub fn PlaylistsSection() -> impl IntoView {
	let playlists = PlaylistsCtx::initialise();

	view! {
		<GridContainer>
			<h1 class="font-semibold text-2xl">{i18n("trending.trending")}</h1>
			<Suspense fallback=move || {
				view! { <PlaceholderCardArray/> }
			}>
				{move || {
					view! {
						<CardGrid>
							<For
								each=move || playlists.playlists.get()
								key=|playlist| playlist.created.clone()
								let:playlist
							>
								<LocalPlaylistPreviewCard playlist=playlist.into()/>
							</For>
						</CardGrid>
					}
				}}

			</Suspense>
		</GridContainer>
	}
}
