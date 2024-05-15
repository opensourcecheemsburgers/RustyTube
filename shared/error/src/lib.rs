use chrono::ParseError;
use gloo::utils::format::JsValueSerdeExt;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, thiserror::Error)]
pub enum RustyTubeError {
	BrowserStorage(String),
	ChronoParse(#[from] chrono::ParseError),
	Csv(String),
	DynInto(String),
	ElementNotFound(String),
	GlooFileRead(String),
	Network(String),
	NoAdaptiveFormat,
	NoAudioFormat,
	NoFileSelected,
	NoLegacyFormat,
	NoThumbnails,
	NoVideoFormat,
	NoVideoQuality,
	NoVideoUrl,
	PlaylistParse,
	Ron(#[from] ron::error::Error),
	RonSpanned(#[from] ron::error::SpannedError),
	SearchArgs,
	SerdeJson(String),
	Tauri(#[from] tauri_sys::error::Error),
	TomlSerialisation(#[from] toml::ser::Error),
	TomlDeserialisation(#[from] toml::de::Error),
	Websys(String),
	Xml(String),
}

impl From<JsValue> for RustyTubeError {
	fn from(value: JsValue) -> Self {
		RustyTubeError::Websys(value.into_serde().unwrap_or_default())
	}
}

impl std::fmt::Display for RustyTubeError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			RustyTubeError::BrowserStorage(error) => {
				write!(f, "Browser Storage Error: {}", error.to_string())
			}
			RustyTubeError::ChronoParse(error) => {
				write!(f, "Time Parse Error: {}", error.to_string())
			}
			RustyTubeError::Csv(error) => write!(f, "Csv Parse Error: {}", error.to_string()),
			RustyTubeError::DynInto(_) => todo!(),
			RustyTubeError::ElementNotFound(_) => todo!(),
			RustyTubeError::GlooFileRead(error) => {
				write!(f, "File Read Error: {}", error.to_string())
			}
			RustyTubeError::Network(error) => write!(f, "Network Error: {}", error.to_string()),
			RustyTubeError::NoAdaptiveFormat => write!(f, "Error: No adaptive formats available."),
			RustyTubeError::NoAudioFormat => write!(f, "Error: No audio formats available."),
			RustyTubeError::NoFileSelected => write!(f, "No file selected."),
			RustyTubeError::NoLegacyFormat => write!(f, "No legacy format available."),
			RustyTubeError::NoThumbnails => write!(f, "Error: No thumbnails available."),
			RustyTubeError::NoVideoFormat => write!(f, "Error: No video formats available."),
			RustyTubeError::NoVideoQuality => write!(f, "Error: No video qualities available."),
			RustyTubeError::NoVideoUrl => write!(f, "Error: No video urls available."),
			RustyTubeError::PlaylistParse => write!(f, "Error: Playlist parse failed."),
			RustyTubeError::Ron(error) => write!(f, "Ron Error: {}", error.to_string()),
			RustyTubeError::RonSpanned(error) => write!(f, "Ron Error: {}", error.to_string()),
			RustyTubeError::SearchArgs => write!(f, "Error: Search args invalid."),
			RustyTubeError::SerdeJson(error) => {
				write!(f, "Serde Json Error: {}", error.to_string())
			}
			RustyTubeError::Tauri(error) => write!(f, "Tauri Error: {}", error.to_string()),
			RustyTubeError::TomlSerialisation(error) => {
				write!(f, "Toml Serialisation Error: {}", error.to_string())
			}
			RustyTubeError::TomlDeserialisation(error) => {
				write!(f, "Toml Deserialisation Error: {}", error.to_string())
			}
			RustyTubeError::Websys(error) => write!(f, "Websys Error: {}", error.to_string()),
			RustyTubeError::Xml(error) => write!(f, "Xml Error: {}", error.to_string()),
		}
	}
}
impl From<serde_json::Error> for RustyTubeError {
	fn from(serde_json_err: serde_json::Error) -> Self {
		Self::SerdeJson(serde_json_err.to_string())
	}
}

impl From<csv::Error> for RustyTubeError {
	fn from(csv_err: csv::Error) -> Self {
		Self::Csv(csv_err.to_string())
	}
}

impl From<gloo::storage::errors::StorageError> for RustyTubeError {
	fn from(storage_error: gloo::storage::errors::StorageError) -> Self {
		Self::BrowserStorage(storage_error.to_string())
	}
}

impl From<reqwasm::Error> for RustyTubeError {
	fn from(reqwasm_error: reqwasm::Error) -> Self {
		Self::Network(reqwasm_error.to_string())
	}
}

impl From<gloo::file::FileReadError> for RustyTubeError {
	fn from(file_read_error: gloo::file::FileReadError) -> Self {
		Self::GlooFileRead(file_read_error.to_string())
	}
}

impl From<serde_xml_rs::Error> for RustyTubeError {
	fn from(xml_error: serde_xml_rs::Error) -> Self {
		Self::Xml(xml_error.to_string())
	}
}
