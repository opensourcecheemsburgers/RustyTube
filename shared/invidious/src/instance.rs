use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::fetch::fetch;

pub const INSTANCES_API_LINK: &str = "https://api.invidious.io/instances.json";

/// # Errors
///
/// - Network errors
pub async fn fetch_instance_info() -> Result<Instances, RustyTubeError> {
	let instances_json = fetch(INSTANCES_API_LINK).await?;
	let instances: Instances = serde_json::from_str(&instances_json)?;
	Ok(instances)
}

pub type Instances = Vec<Instance>;
pub type Instance = (String, InstanceInfo);

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
	pub version: Option<String>,
	pub software: Option<Software>,
	pub open_registrations: Option<bool>,
	pub usage: Option<Usage>,
	pub metadata: Option<Metadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Software {
	pub name: Option<String>,
	pub version: Option<String>,
	pub branch: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
	pub users: Option<Users>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Users {
	pub total: Option<i64>,
	#[serde(rename = "activeHalfyear")]
	pub active_half_year: Option<i64>,
	pub active_month: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
	pub updated_at: Option<i64>,
	pub last_channel_refreshed_at: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
	pub monitor_id: Option<i64>,
	pub created_at: Option<i64>,
	pub status_class: Option<String>,
	pub name: Option<String>,
	pub url: Option<Value>,
	#[serde(rename = "type")]
	pub type_field: Option<String>,
	pub daily_ratios: Option<Vec<DailyRatio>>,
	#[serde(rename = "90dRatio")]
	pub quarterly_ratio: Option<QuarterlyRatio>,
	#[serde(rename = "30dRatio")]
	pub monthly_ratio: Option<MonthlyRatio>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyRatio {
	pub ratio: Option<String>,
	pub label: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuarterlyRatio {
	pub ratio: Option<String>,
	pub label: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyRatio {
	pub ratio: Option<String>,
	pub label: Option<String>,
}
