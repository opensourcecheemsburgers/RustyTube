use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{
	common::{CommonImage, CommonPlaylist, CommonVideo},
	fetch::fetch,
	hidden::{CountryCode, RelatedChannel},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
	#[serde(rename = "author")]
	pub name: String,
	#[serde(rename = "authorId")]
	pub id: String,
	#[serde(rename = "authorUrl")]
	pub url: String,
	#[serde(rename = "authorBanners")]
	pub banners: Vec<CommonImage>,
	#[serde(rename = "authorThumbnails")]
	pub thumbnails: Vec<CommonImage>,
	#[serde(rename = "subCount")]
	pub subscribers: u32,
	pub total_views: u64,
	pub joined: u64,
	pub auto_generated: bool,
	#[serde(rename = "isFamilyFriendly")]
	pub family_friendly: bool,
	pub description: String,
	pub description_html: String,
	pub allowed_regions: Vec<CountryCode>,
	pub latest_videos: Vec<CommonVideo>,
	pub related_channels: Vec<RelatedChannel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelThumb {
	#[serde(rename = "author")]
	pub name: String,
	#[serde(rename = "authorId")]
	pub id: String,
	#[serde(rename = "authorThumbnails")]
	pub thumbnails: Vec<CommonImage>,
}

impl Channel {
	/// # Errors
	///
	/// - Network errors.
	/// - Serde errors.
	pub async fn fetch_channel(
		server: &str,
		id: &str,
		lang: &str,
	) -> Result<Self, RustyTubeError> {
		let channel_url = format!("{server}/api/v1/channels/{id}?hl={lang}");
		let channel_json: String = fetch(&channel_url).await?;
		let channel: Self = serde_json::from_str(&channel_json)?;
		Ok(channel)
	}

	/// # Errors
	///
	/// - Network errors.
	/// - Serde errors.
	pub async fn fetch_channel_thumbnails(
		server: &str,
		id: &str,
	) -> Result<Vec<CommonImage>, RustyTubeError> {
		let thumbnails_url =
			format!("{server}/api/v1/channels/{id}?fields=authorThumbnails");
		let thumbnails_json = fetch(&thumbnails_url).await?;
		let thumbnails = serde_json::from_str(&thumbnails_json)?;
		Ok(thumbnails)
	}

	/// # Errors
	///
	/// - Network errors.
	/// - Serde errors.
	pub async fn fetch_channel_thumb(
		server: &str,
		id: &str,
	) -> Result<ChannelThumb, RustyTubeError> {
		let thumbnails_url =
			format!("{server}/api/v1/channels/{id}?fields=author,authorId,authorThumbnails");
		let thumbnails_json = fetch(&thumbnails_url).await?;
		let thumbnails = serde_json::from_str(&thumbnails_json)?;
		Ok(thumbnails)
	}

	/// # Errors
	///
	/// - Network errors.
	/// - Serde errors.
	pub async fn fetch_channel_videos(
		server: &str,
		id: &str,
		continuation: Option<&str>,
		lang: &str,
	) -> Result<ChannelVideos, RustyTubeError> {
		let videos_url = continuation.map_or_else(
			|| format!("{server}/api/v1/channels/{id}/videos?hl={lang}"),
			|continuation| {
				format!(
					"{server}/api/v1/channels/{id}/videos?continuation={continuation}&hl={lang}"
				)
			},
		);
		let videos_json = fetch(&videos_url).await?;
		let videos = serde_json::from_str(&videos_json)?;
		Ok(videos)
	}

	/// # Errors
	///
	/// - Network errors.
	/// - Serde errors.
	pub async fn fetch_channel_shorts(
		server: &str,
		id: &str,
		continuation: Option<&str>,
		lang: &str,
	) -> Result<ChannelShorts, RustyTubeError> {
		let shorts_url = continuation.map_or(
			format!("{server}/api/v1/channels/{id}/shorts?hl={lang}"),
			|continuation| {
				format!(
					"{server}/api/v1/channels/{id}/shorts?continuation={continuation}&hl={lang}"
				)
			},
		);
		let shorts_json = fetch(&shorts_url).await?;
		let shorts = serde_json::from_str(&shorts_json)?;
		Ok(shorts)
	}

	/// # Errors
	///
	/// - Network errors.
	/// - Serde errors.
	pub async fn fetch_channel_livestreams(
		server: &str,
		id: &str,
		continuation: Option<&str>,
		lang: &str,
	) -> Result<ChannelLivestreams, RustyTubeError> {
		let livestreams_url = continuation.map_or_else(
			|| format!("{server}/api/v1/channels/{id}/streams?hl={lang}"),
			|continuation| {
				format!(
					"{server}/api/v1/channels/{id}/streams?continuation={continuation}&hl={lang}"
				)
			},
		);
		let livestreams_json = fetch(&livestreams_url).await?;
		let livestreams = serde_json::from_str(&livestreams_json)?;
		Ok(livestreams)
	}

	/// # Errors
	///
	/// - Network errors.
	/// - Serde errors.
	pub async fn fetch_channel_playlists(
		server: &str,
		id: &str,
		continuation: Option<&str>,
		lang: &str,
	) -> Result<ChannelPlaylists, RustyTubeError> {
		let playlists_url = continuation.map_or_else(
			|| format!("{server}/api/v1/channels/{id}/playlists?hl={lang}"),
			|continuation| {
				format!(
					"{server}/api/v1/channels/{id}/playlists?continuation={continuation}&hl={lang}"
				)
			},
		);
		let channel_videos_json = fetch(&playlists_url).await?;
		let channel_videos = serde_json::from_str(&channel_videos_json)?;
		Ok(channel_videos)
	}
}

impl PartialEq for Channel {
	fn eq(&self, other: &Self) -> bool {
		self.id.eq(&other.id)
	}
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelVideos {
	pub videos: Vec<CommonVideo>,
	pub continuation: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelShorts {
	#[serde(rename = "videos")]
	pub shorts: Vec<CommonVideo>,
	pub continuation: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelLivestreams {
	#[serde(rename = "videos")]
	pub livestreams: Vec<CommonVideo>,
	pub continuation: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelPlaylists {
	pub playlists: Vec<CommonPlaylist>,
	pub continuation: Option<String>,
}
