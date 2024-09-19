use gloo::utils::format::JsValueSerdeExt;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, thiserror::Error)]
pub enum RustyTubeError {
	BrowserStorage(String),
	ChronoParse(#[from] chrono::ParseError),
	Csv(String),
	DateTime,
	DynInto,
	ElementNotFound,
	GlooFileRead(String),
	LangCode,
	Network(String),
	NoAdaptiveFormat,
	NoAudioFormat,
	NoFileSelected,
	NoLegacyFormat,
	NoThumbnails,
	NoVideoFormat,
	NoVideoQuality,
	NoVideoUrl,
	NoPerformance,
	NoWindow,
	PlaylistParse,
	Ron(#[from] ron::error::Error),
	RonSpanned(#[from] ron::error::SpannedError),
	SearchArgs,
	SerdeJson(String),
	TargetNotFound,
	Tauri(#[from] tauri_sys::error::Error),
	TomlSerialisation(#[from] toml::ser::Error),
	TomlDeserialisation(#[from] toml::de::Error),
	Websys(String),
	Xml(String),
}

impl From<JsValue> for RustyTubeError {
	fn from(value: JsValue) -> Self {
		Self::Websys(JsValueSerdeExt::into_serde(&value).unwrap_or_default())
	}
}

impl std::fmt::Display for RustyTubeError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::BrowserStorage(error) => {
				write!(f, "Browser Storage Error: {error}")
			}
			Self::ChronoParse(error) => {
				write!(f, "Time Parse Error: {error}")
			}
			Self::Csv(error) => write!(f, "Csv Parse Error: {error}"),
			Self::DateTime => write!(f, "Date time error."),
			Self::DynInto => write!(f, "Dynamic conversion error."),
			Self::ElementNotFound => write!(f, "Element not found error."),
			Self::GlooFileRead(error) => {
				write!(f, "File Read Error: {error}")
			}
			Self::LangCode => write!(f, "Unknown lang code."),
			Self::Network(error) => write!(f, "Network Error: {error}"),
			Self::NoAdaptiveFormat => {
				write!(f, "Error: No adaptive formats available.")
			}
			Self::NoAudioFormat => {
				write!(f, "Error: No audio formats available.")
			}
			Self::NoFileSelected => write!(f, "No file selected."),
			Self::NoLegacyFormat => write!(f, "No legacy format available."),
			Self::NoThumbnails => write!(f, "Error: No thumbnails available."),
			Self::NoVideoFormat => {
				write!(f, "Error: No video formats available.")
			}
			Self::NoVideoQuality => {
				write!(f, "Error: No video qualities available.")
			}
			Self::NoVideoUrl => write!(f, "Error: No video urls available."),
			Self::NoPerformance => {
				write!(f, "Error: Perfomance not available.")
			}
			Self::NoWindow => write!(f, "Error: Window not available."),
			Self::PlaylistParse => write!(f, "Error: Playlist parse failed."),
			Self::Ron(error) => write!(f, "Ron Error: {error}"),
			Self::RonSpanned(error) => write!(f, "Ron Error: {error}"),
			Self::SearchArgs => write!(f, "Error: Search args invalid."),
			Self::SerdeJson(error) => {
				write!(f, "Serde Json Error: {error}")
			}
			Self::TargetNotFound => write!(f, "Error: target not found."),
			Self::Tauri(error) => write!(f, "Tauri Error: {error}"),
			Self::TomlSerialisation(error) => {
				write!(f, "Toml Serialisation Error: {error}")
			}
			Self::TomlDeserialisation(error) => {
				write!(f, "Toml Deserialisation Error: {error}")
			}
			Self::Websys(error) => write!(f, "Websys Error: {error}"),
			Self::Xml(error) => write!(f, "Xml Error: {error}"),
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
