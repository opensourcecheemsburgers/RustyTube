use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum RustyTubeLocale {
	EN_US,
	FR_FR,
}

impl RustyTubeLocale {
	pub fn id(&self) -> String {
		match &self {
			RustyTubeLocale::EN_US => "en-US".to_string(),
			RustyTubeLocale::FR_FR => "fr-FR".to_string(),
		}
	}

	pub fn human_name(&self) -> String {
		match &self {
			RustyTubeLocale::EN_US => "English (US)".to_string(),
			RustyTubeLocale::FR_FR => "Français (France)".to_string(),
		}
	}

	pub fn from_str(lang: &str) -> Self {
		match lang {
			"en-US" => Self::EN_US,
			"fr-FR" => Self::FR_FR,
			_ => Self::EN_US,
		}
	}

	pub fn to_invidious_lang(&self) -> String {
		match self {
			RustyTubeLocale::EN_US => "en".to_string(),
			RustyTubeLocale::FR_FR => "fr".to_string(),
		}
	}
	pub fn to_num_fmt(&self) -> num_format::Locale {
		match self {
			RustyTubeLocale::EN_US => num_format::Locale::en,
			RustyTubeLocale::FR_FR => num_format::Locale::fr,
		}
	}
}
