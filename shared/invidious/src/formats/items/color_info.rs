use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorInfo {
	pub primaries: Option<String>,
	pub transfer_characteristics: String,
	pub matrix_coefficients: Option<String>,
}
