use crate::{fetch, CommonThumbnail, CommonVideo, ChannelVideos};
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

/// Fetching feeds via RSS is currently broken due to CORS restrictions in regular browsers.
impl Feed {
	pub async fn fetch_videos_from_feed(
		server: &str,
		id: &str,
	) -> Result<ChannelVideos, RustyTubeError> {
		let playlist_id = id.replace("UC", "UULF");
		let url = format!("{}/feed/playlist/{}/", server, playlist_id);
		let response = fetch(&url).await?;
		let feed: Feed = serde_xml_rs::from_str(&response)?;
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
	pub views: u64,
}

impl Into<ChannelVideos> for Feed {
	fn into(self) -> ChannelVideos {
		let mut videos = Vec::new();
		self.entries.into_iter().for_each(|entry| {
			videos.push(entry.into());
		});
		ChannelVideos { videos, continuation: None }
	}
}

impl Into<CommonVideo> for Entry {
	fn into(self) -> CommonVideo {
		let title = self.title;
		let id = self.id;
		let author = self.author.name;
		let author_id = self.channel_id;
		let author_url = self.author.uri;

		let thumbnail = self.group.thumbnail;
		let mut thumbnails = Vec::new();
		let common_thumbnail = CommonThumbnail {
			quality: format!("{}x{}", thumbnail.width, thumbnail.height),
			url: thumbnail.url,
			width: thumbnail.width,
			height: thumbnail.height,
		};
		thumbnails.push(common_thumbnail);

		let description = self.group.description;
		let description_html = "".to_string();
		let views = self.group.community.statistics.views;
		let views_text = "".to_string();
		let author_verified = false;
		let length = 0;
		let published = utils::get_published_time_ms(&self.published).unwrap();
		let published_text = utils::get_published_time(&self.published).unwrap();
		let premiere_timestamp = 0;
		let live = false;
		let premium = false;
		let upcoming = false;

		CommonVideo {
			title,
			id,
			author,
			author_id,
			author_url,
			thumbnails,
			description,
			description_html,
			views,
			length,
			published,
			published_text,
			premiere_timestamp,
			live,
			premium,
			upcoming,
			author_verified,
			views_text,
		}
	}
}



