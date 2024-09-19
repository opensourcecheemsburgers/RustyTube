use leptos::{component, view, For, IntoView, Props, SignalGet, Suspense};

use crate::{
	components::{
		CardGrid, GridContainer, LocalPlaylistPreviewCard, PlaceholderCardArray,
	},
	resources::PlaylistsCtx,
	utils::i18n,
};

#[component]
pub fn PlaylistsSection() -> impl IntoView {
	let playlists = PlaylistsCtx::initialise();

	view! {
		<GridContainer>
			<h1 class="text-2xl font-semibold">{i18n("trending.trending")}</h1>
			<Suspense fallback=move || {
				view! { <PlaceholderCardArray/> }
			}>
				{move || {
					view! {
						<CardGrid>
							<For
								each=move || playlists.playlists.get()
								key=|playlist| playlist.created
								let:playlist
							>
								<LocalPlaylistPreviewCard playlist=playlist/>
							</For>
						</CardGrid>
					}
				}}

			</Suspense>
		</GridContainer>
	}
}
