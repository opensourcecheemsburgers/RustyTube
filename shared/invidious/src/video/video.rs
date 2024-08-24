use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

use crate::{
	common::{CommonImage, CommonThumbnail},
	fetch::fetch,
	formats::{AdaptiveFormat, LegacyFormat},
	hidden::{Caption, CountryCode, Storyboard, VideoShort},
};

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
	pub title: String,
	#[serde(rename = "videoId")]
	pub id: String,
	#[serde(rename = "videoThumbnails")]
	pub thumbnails: Vec<CommonThumbnail>,
	pub storyboards: Vec<Storyboard>,
	pub description: String,
	#[serde(rename = "descriptionHtml")]
	pub description_html: String,
	pub published: u64,
	#[serde(rename = "publishedText")]
	pub published_text: String,

	pub keywords: Vec<String>,
	#[serde(rename = "viewCount")]
	pub views: u64,
	#[serde(rename = "likeCount")]
	pub likes: u32,
	#[serde(rename = "dislikeCount")]
	pub dislikes: u32,

	pub paid: bool,
	pub premium: bool,
	#[serde(rename = "isFamilyFriendly")]
	pub family_friendly: bool,
	#[serde(rename = "allowedRegions")]
	pub allowed_regions: Vec<CountryCode>,
	pub genre: String,
	#[serde(rename = "genreUrl")]
	pub genre_url: Option<String>,

	pub author: String,
	#[serde(rename = "authorId")]
	pub author_id: String,
	#[serde(rename = "authorUrl")]
	pub author_url: String,
	#[serde(rename = "authorThumbnails")]
	pub author_thumbnails: Vec<CommonImage>,

	#[serde(rename = "subCountText")]
	pub sub_count_text: String,
	#[serde(rename = "lengthSeconds")]
	pub length: u32,
	#[serde(rename = "allowRatings")]
	pub allow_ratings: bool,
	pub rating: f32,
	#[serde(rename = "isListed")]
	pub listed: bool,
	#[serde(rename = "liveNow")]
	pub live: bool,
	#[serde(rename = "isUpcoming")]
	pub upcoming: bool,
	#[serde(rename = "premiereTimestamp")]
	#[serde(default)]
	pub premiere_timestamp: u64,
	#[serde(rename = "dashUrl")]
	pub dash: String,

	#[serde(rename = "adaptiveFormats")]
	pub adaptive_formats: Vec<AdaptiveFormat>,
	#[serde(rename = "formatStreams")]
	pub format_streams: Vec<LegacyFormat>,

	pub captions: Vec<Caption>,

	#[serde(rename = "recommendedVideos")]
	pub recommended_videos: Vec<VideoShort>,
}

impl Video {
	pub async fn fetch_video(
		server: &str,
		id: &str,
		lang: &str,
	) -> Result<Self, RustyTubeError> {
		let video_url = format!("{server}/api/v1/videos/{id}?hl={lang}");
		let video_json = fetch(&video_url).await?;
		let video = serde_json::from_str(&video_json)?;
		Ok(video)
	}
}

impl PartialEq for Video {
	fn eq(&self, other: &Self) -> bool {
		self.id.eq(&other.id)
	}
}
