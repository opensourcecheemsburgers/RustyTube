use std::str::FromStr;

use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum RustyTubeLocale {
	EN_US,
	FR_FR,
	AR_SY,
}

pub const ENGLISH_US_LANG_CODE: &str = "en-US";
pub const FRENCH_LANG_CODE: &str = "fr-FR";
pub const ARABIC_LANG_CODE: &str = "ar-SY";

pub const ENGLISH_US_HUMAN_NAME: &str = "English (US)";
pub const FRENCH_HUMAN_NAME: &str = "Français (France)";
pub const ARABIC_HUMAN_NAME: &str = "العربية (سوريا)";

pub const ENGLISH_US_INVIDIOUS_CODE: &str = "en";
pub const FRENCH_INVIDIOUS_CODE: &str = "fr";
pub const ARABIC_INVIDIOUS_CODE: &str = "ar";

impl RustyTubeLocale {
	#[allow(clippy::missing_const_for_fn)]
	#[allow(clippy::wildcard_in_or_patterns)]
	pub fn from_lang_code_str(lang_code_str: &str) -> Self {
		match lang_code_str {
			_ | ENGLISH_US_LANG_CODE => Self::EN_US,
			FRENCH_LANG_CODE => Self::FR_FR,
			ARABIC_LANG_CODE => Self::AR_SY,
		}
	}
	pub const fn id(&self) -> &'static str {
		match &self {
			Self::EN_US => ENGLISH_US_LANG_CODE,
			Self::FR_FR => FRENCH_LANG_CODE,
			Self::AR_SY => ARABIC_LANG_CODE,
		}
	}

	pub const fn human_name(&self) -> &'static str {
		match &self {
			Self::EN_US => ENGLISH_US_HUMAN_NAME,
			Self::FR_FR => FRENCH_HUMAN_NAME,
			Self::AR_SY => ARABIC_HUMAN_NAME,
		}
	}

	pub const fn to_invidious_lang(&self) -> &'static str {
		match self {
			Self::EN_US => ENGLISH_US_INVIDIOUS_CODE,
			Self::FR_FR => FRENCH_INVIDIOUS_CODE,
			Self::AR_SY => ARABIC_INVIDIOUS_CODE,
		}
	}
	pub const fn to_num_fmt(&self) -> num_format::Locale {
		match self {
			Self::EN_US => num_format::Locale::en,
			Self::FR_FR => num_format::Locale::fr,
			Self::AR_SY => num_format::Locale::ar_SY,
		}
	}

	pub const fn is_rtl_lang(&self) -> bool {
		match self {
			Self::EN_US | Self::FR_FR => false,
			Self::AR_SY => true,
		}
	}
}
