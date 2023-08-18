use serde::{Deserialize, Serialize};
use serde_json::Value;
use rustytube_error::RustyTubeError;
use crate::fetch::fetch;

pub const INSTANCES_API_LINK: &'static str = "https://api.invidious.io/instances.json";

pub async fn fetch_instance_info() -> Result<Instances, RustyTubeError> {
	let instances_json = fetch(INSTANCES_API_LINK).await?;
	let instances: Instances = serde_json::from_str(&instances_json)?;
	Ok(instances)
}

pub type Instances = Vec<Instance>;
pub type Instance = (String, InstanceInfo);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceInfo {
	pub flag: String,
	pub region: String,
	pub stats: Option<Stats>,
	pub cors: Option<bool>,
	pub api: Option<bool>,
	#[serde(rename = "type")]
	pub type_field: String,
	pub uri: String,
	pub monitor: Option<Monitor>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
	pub version: String,
	pub software: Software,
	pub open_registrations: bool,
	pub usage: Usage,
	pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Software {
	pub name: String,
	pub version: String,
	pub branch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
	pub users: Users,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Users {
	pub total: i64,
	#[serde(rename = "activeHalfyear")]
	pub active_half_year: i64,
	pub active_month: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
	pub updated_at: i64,
	pub last_channel_refreshed_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
	pub monitor_id: i64,
	pub created_at: i64,
	pub status_class: String,
	pub name: String,
	pub url: Value,
	#[serde(rename = "type")]
	pub type_field: String,
	pub daily_ratios: Vec<DailyRatio>,
	#[serde(rename = "90dRatio")]
	pub quarterly_ratio: QuarterlyRatio,
	#[serde(rename = "30dRatio")]
	pub monthly_ratio: MonthlyRatio,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyRatio {
	pub ratio: String,
	pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuarterlyRatio {
	pub ratio: String,
	pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyRatio {
	pub ratio: String,
	pub label: String,
}
