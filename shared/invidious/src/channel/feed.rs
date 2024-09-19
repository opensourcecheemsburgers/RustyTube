use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{fetch, ChannelVideos, CommonThumbnail, CommonVideo};

/// Fetching feeds via RSS is currently broken due to CORS restrictions in regular browsers.
impl Feed {
	/// # Errors
	///
	/// - Network errors.
	/// - Serde errors.
	pub async fn fetch_videos_from_feed(
		server: &str,
		id: &str,
	) -> Result<ChannelVideos, RustyTubeError> {
		let playlist_id = id.replace("UC", "UULF");
		let url = format!("{server}/feed/playlist/{playlist_id}/");
		let response = fetch(&url).await?;
		let feed = serde_xml_rs::from_str::<Self>(&response)?;
		Ok(feed.into())
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
	pub id: String,
	pub playlist_id: String,
	pub channel_id: String,
	pub title: String,
	pub author: Author,
	pub published: String,
	#[serde(rename = "entry", default)]
	pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
	pub name: String,
	pub uri: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
	pub id: String,
	pub video_id: String,
	pub channel_id: String,
	pub title: String,
	pub author: Author,
	pub published: String,
	pub updated: String,
	pub group: Media,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
	pub title: String,
	pub thumbnail: Thumbnail,
	pub description: String,
	pub community: Community,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
	pub url: String,
	pub height: u32,
	pub width: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Community {
	pub star_rating: StarRating,
	pub statistics: Statistics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StarRating {
	pub count: u32,
	pub average: f32,
	pub min: u32,
	pub max: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
	pub views: u64,
}

impl From<Feed> for ChannelVideos {
	fn from(val: Feed) -> Self {
		let mut videos = Vec::new();
		val.entries.into_iter().for_each(|entry| {
			videos.push(entry.into());
		});
		Self { videos, continuation: None }
	}
}

impl From<Entry> for CommonVideo {
	fn from(val: Entry) -> Self {
		let title = val.title;
		let id = val.id;
		let author = val.author.name;
		let author_id = val.channel_id;
		let author_url = val.author.uri;

		let thumbnail = val.group.thumbnail;
		let mut thumbnails = Vec::new();
		let common_thumbnail = CommonThumbnail {
			quality: format!("{}x{}", thumbnail.width, thumbnail.height),
			url: thumbnail.url,
			width: thumbnail.width,
			height: thumbnail.height,
		};
		thumbnails.push(common_thumbnail);

		let description = val.group.description;
		let description_html = String::new();
		let views = val.group.community.statistics.views;
		let views_text = String::new();
		let author_verified = false;
		let length = 0;
		let published =
			utils::get_published_time_ms(&val.published).unwrap_or_default();
		let published_text =
			utils::get_published_time(&val.published).unwrap_or_default();
		let premiere_timestamp = 0;
		let live = false;
		let premium = false;
		let upcoming = false;

		Self {
			title,
			id,
			author,
			author_id,
			author_url,
			author_verified,
			thumbnails,
			description,
			description_html,
			views,
			views_text,
			length,
			published,
			published_text,
			premiere_timestamp,
			live,
			premium,
			upcoming,
		}
	}
}
