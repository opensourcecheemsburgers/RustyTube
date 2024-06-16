use gloo::storage::{LocalStorage, Storage};
use invidious::LocalPlaylist;
use leptos::{RwSignal, SignalGet, SignalUpdate};
use locales::RustyTubeLocale;
use rustytube_error::RustyTubeError;

use crate::contexts::{NetworkConfigCtx, RegionConfigCtx};

static PLAYLISTS_KEY: &str = "playlists";

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct PlaylistsCtx {
	pub playlists: RwSignal<Vec<LocalPlaylist>>,
}

impl PlaylistsCtx {
	pub fn initialise() -> Self {
		Self {
			playlists: RwSignal::new(
				get_playlists(PLAYLISTS_KEY).unwrap_or_default(),
			),
		}
	}

	pub fn add_playlist(
		&self,
		playlist: LocalPlaylist,
	) -> Result<(), RustyTubeError> {
		self.playlists.update(|playlists| {
			playlists.push(playlist);
			save_playlists(playlists);
		});
		Ok(())
	}
}

fn get_playlists(
	key: &'static str,
) -> Result<Vec<LocalPlaylist>, RustyTubeError> {
	Ok(LocalStorage::get::<Vec<LocalPlaylist>>(key)?)
}

pub fn save_playlists(playlists: &mut Vec<LocalPlaylist>) {
	for playlist in playlists {
		playlist.save();
	}
}
