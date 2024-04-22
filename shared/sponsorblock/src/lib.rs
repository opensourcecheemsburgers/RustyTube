pub static SPONSORBLOCK_API: &'static str = "https://sponsor.ajay.app/api";

use std::{error::Error, future::Future};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Query {
	#[serde(rename = "videoID")]
	video_id: String,
	#[serde(rename = "requiredSegments")]
	required_segments: Option<Vec<String>>,
	categories: Option<Vec<Category>>,
	#[serde(rename = "actionTypes")]
	actions: Option<Vec<Action>>,
	service: Option<String>,
}

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
pub enum Category {
	#[default]
	Sponsor,
	SelfPromotion,
	Interaction,
	Intro,
	Outro,
	Preview,
	OffTopicMusic,
	Filler,
}

impl ToString for Category {
	fn to_string(&self) -> String {
		match self {
			Category::Sponsor => "sponsor".to_string(),
			Category::SelfPromotion => "selfpromo".to_string(),
			Category::Interaction => "interaction".to_string(),
			Category::Intro => "intro".to_string(),
			Category::Outro => "outro".to_string(),
			Category::Preview => "preview".to_string(),
			Category::OffTopicMusic => "music_offtopic".to_string(),
			Category::Filler => "filler".to_string(),
		}
	}
}

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
pub enum Action {
	#[default]
	Skip,
	Mute,
	Full,
	PointOfInterest,
	Chapter,
}

impl ToString for Action {
	fn to_string(&self) -> String {
		match self {
			Action::Skip => "skip".to_string(),
			Action::Mute => "mute".to_string(),
			Action::Full => "full".to_string(),
			Action::PointOfInterest => "poi".to_string(),
			Action::Chapter => "chapter".to_string(),
		}
	}
}

impl Query {
	pub fn create(
		video_id: String,
		required_segments: Option<Vec<String>>,
		categories: Option<Vec<Category>>,
		actions: Option<Vec<Action>>,
		service: Option<String>,
	) -> Self {
		Self { video_id, required_segments, categories, actions, service }
	}

	pub fn build(video_id: String) -> Self {
		Self { video_id, required_segments: None, categories: None, actions: None, service: None }
	}

	pub fn set_required_segments(&mut self, required_segments: Option<Vec<String>>) -> Self {
		self.required_segments = required_segments;
		self.clone()
	}
	pub fn set_categories(&mut self, categories: Option<Vec<Category>>) -> Self {
		self.categories = categories;
		self.clone()
	}
	pub fn set_actions(&mut self, actions: Option<Vec<Action>>) -> Self {
		self.actions = actions;
		self.clone()
	}
	pub fn set_service(&mut self, service: Option<String>) -> Self {
		self.service = service;
		self.clone()
	}

	pub fn url(&self) -> String {
		let required_segments = self
			.required_segments
			.as_ref()
			.map(|required_segments| format!("requiredSegments=[{}]", required_segments.join(",")));

		let categories = self.categories.as_ref().map(|categories| {
			format!(
				"categories=[{}]",
				categories.iter().map(|cat| cat.to_string()).collect::<Vec<String>>().join(",")
			)
		});

		let actions = self.actions.as_ref().map(|actions| {
			format!(
				"actionTypes=[{}]",
				actions.iter().map(|cat| cat.to_string()).collect::<Vec<String>>().join(",")
			)
		});

		let service = self.service.as_ref().map(|service| format!("&service={}", service));

		format!(
			"{}/skipSegments?videoID={}{}{}{}{}",
			SPONSORBLOCK_API,
			self.video_id,
			required_segments.unwrap_or_default(),
			categories.unwrap_or_default(),
			actions.unwrap_or_default(),
			service.unwrap_or_default()
		)
	}

	pub async fn send_query(&self) -> Result<Option<Response>, Box<dyn Error>> {
		let response = gloo::net::http::Request::get(&self.url()).send().await?;

		match response.status() == 404 {
			true => Ok(None),
			false => {
				let response_text = response.text().await?;
				Ok(Some(serde_json::from_str::<Response>(&response_text)?))
			}
		}
	}
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Segment {
	segment: (f64, f64),
	#[serde(rename = "UUID")]
	uuid: String,
	category: String,
	#[serde(rename = "videoDuration")]
	video_duration: f64,
	#[serde(rename = "actionType")]
	action: String,
	locked: u8,
	votes: i64,
	description: String,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Response {
	response: Vec<Segment>,
}

#[cfg(test)]
mod tests {
	use std::{error::Error, future::Future};

	use crate::{Query, Response};

	use wasm_bindgen_test::*;

	pub static TEST_VIDEOS: &[&'static str; 61] = &[
		"wsmHCfSZM70",
		"RE0f4ed5N24",
		"Bu6PxzNR3dY",
		"r22tyT77vOw",
		"SKIXCPn2xB0",
		"HEbaKDkzomI",
		"yLy3ygqA5yg",
		"s1fxZ-VWs2U",
		"gIMOtNzjHL4",
		"69dCWRvIzyo",
		"bc8Okr4cgL4",
		"1EIlcYCfEIE",
		"sB1XQYDbzOE",
		"QsM6b5yix0U",
		"seoaDLWuHtU",
		"Al93JD5GExY",
		"kaf3pdJ_Cow",
		"i9TJWsuzBLU",
		"CcHevgjAnV0",
		"8BxVi6YiicQ",
		"DNfj2BxGIxA",
		"X5OIucMnw7M",
		"n3XTZde8ZvQ",
		"FwdDAZruMKk",
		"-duJtlw394U",
		"AfzwEF5yr3k",
		"YjkEVrJP7jI",
		"fpayOqZNWUo",
		"TNZk-xnxIYE",
		"iQr1EZ3rLOM",
		"WXV-zB3EfNw",
		"qz7NHaCspzg",
		"YE431SYO2Is",
		"YUMtJ6K43K8",
		"2fpZbH4BNsI",
		"8V8uQbIFlh0",
		"tVvbLS2Bm8c",
		"gMULrCIT6QY",
		"1r82NBk3aKM",
		"JoaIoctknLk",
		"jv3zuQ-forQ",
		"M42qWWi4y6k",
		"JAKm3-ijEBo",
		"5kJv-oSajto",
		"mtaQroi75M0",
		"nCS4BtJ34-o",
		"8ZrNRk9OUGA",
		"j6Gf482ZjSg",
		"Qa6y_CiyAMA",
		"mY-Yc1B6vdk",
		"aeifzxaDOVo",
		"4QY6ADlspTQ",
		"3wCexOqw-h4",
		"VvMjFXwL9Uw",
		"BH5ghaSLVUc",
		"MuP-9O7gNIc",
		"9vjVT-Pp4R8",
		"hDWeJnH4Wys",
		"msiKpFV8HNY",
		"FufN3c68nPg",
		"CW1CLcT83as",
	];

	// #[cfg(not(target_arch = "wasm-32"))]
	// #[test]
	// pub fn collect_query_responses() {
	// 	let client = reqwest::blocking::Client::new();
	// 	for test_video in TEST_VIDEOS {
	// 		let resp = client
	// 			.get(Query::build(test_video.to_string()).url())
	// 			.send()
	// 			.unwrap()
	// 			.text()
	// 			.unwrap();

	// 		serde_json::from_str::<Response>(&resp).unwrap();

	// 		std::fs::write(format!("test_files/{}.json", test_video), resp).unwrap();
	// 	}
	// }

	wasm_bindgen_test_configure!(run_in_browser);

	#[wasm_bindgen_test]
	pub async fn fetch_response() {
		for test_video in TEST_VIDEOS {
			let query = Query::build(test_video.to_string());
			query.send_query().await.unwrap();
		}
	}
}
