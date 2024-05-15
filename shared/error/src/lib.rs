use gloo::utils::format::JsValueSerdeExt;
use wasm_bindgen::JsValue;

#[derive(Debug, thiserror::Error)]
pub enum RustyTubeError {
	Anyhow(#[from] anyhow::Error),
	BrowserStorage(#[from] gloo::storage::errors::StorageError),
	ChronoParse(#[from] chrono::ParseError),
	Csv(#[from] csv::Error),
	DynInto(String),
	ElementNotFound(String),
	GlooFileRead(#[from] gloo::file::FileReadError),
	Network(#[from] reqwasm::Error),
	NoAdaptiveFormat,
	NoAudioFormat,
	NoFileSelected,
	NoLegacyFormat,
	NoThumbnails,
	NoVideoFormat,
	NoVideoQuality,
	NoVideoUrl,
	PlaylistParse,
	Ron(#[from] ron::Error),
	SearchArgs,
	SerdeJson(#[from] serde_json::Error),
	Tauri(#[from] tauri_sys::error::Error),
	TomlSerialisation(#[from] toml::ser::Error),
	TomlDeserialisation(#[from] toml::de::Error),
	Websys(String),
	Xml(#[from] serde_xml_rs::Error),
}

impl Clone for RustyTubeError {
	fn clone(&self) -> Self {
		todo!()
	}
}

impl From<JsValue> for RustyTubeError {
	fn from(value: JsValue) -> Self {
		RustyTubeError::Websys(value.into_serde::<String>().unwrap_or_default())
	}
}

impl std::fmt::Display for RustyTubeError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			RustyTubeError::Anyhow(error) => write!(f, "Error:/n/n{}", error.to_string()),
			RustyTubeError::BrowserStorage(error) => {
				write!(f, "Browser Storage Error:/n/n{}", error.to_string())
			}
			RustyTubeError::ChronoParse(error) => {
				write!(f, "Time Parse Error:/n/n{}", error.to_string())
			}
			RustyTubeError::Csv(error) => write!(f, "Csv Parse Error:/n/n{}", error.to_string()),
			RustyTubeError::DynInto(_) => todo!(),
			RustyTubeError::ElementNotFound(_) => todo!(),
			RustyTubeError::GlooFileRead(error) => {
				write!(f, "File Read Error:/n/n{}", error.to_string())
			}
			RustyTubeError::Network(error) => write!(f, "Network Error:/n/n{}", error.to_string()),
			RustyTubeError::NoAdaptiveFormat => write!(f, "Error: No adaptive formats available."),
			RustyTubeError::NoAudioFormat => write!(f, "Error: No audio formats available."),
			RustyTubeError::NoFileSelected => write!(f, "No file selected."),
			RustyTubeError::NoLegacyFormat => write!(f, "No legacy format available."),
			RustyTubeError::NoThumbnails => write!(f, "Error: No thumbnails available."),
			RustyTubeError::NoVideoFormat => write!(f, "Error: No video formats available."),
			RustyTubeError::NoVideoQuality => write!(f, "Error: No video qualities available."),
			RustyTubeError::NoVideoUrl => write!(f, "Error: No video urls available."),
			RustyTubeError::PlaylistParse => write!(f, "Error: Playlist parse failed."),
			RustyTubeError::Ron(error) => write!(f, "Ron Error:/n/n{}", error.to_string()),
			RustyTubeError::SearchArgs => write!(f, "Error: Search args invalid."),
			RustyTubeError::SerdeJson(error) => {
				write!(f, "Serde Json Error:/n/n{}", error.to_string())
			}
			RustyTubeError::Tauri(error) => write!(f, "Tauri Error:/n/n{}", error.to_string()),
			RustyTubeError::TomlSerialisation(error) => {
				write!(f, "Toml Serialisation Error:/n/n{}", error.to_string())
			}
			RustyTubeError::TomlDeserialisation(error) => {
				write!(f, "Toml Deserialisation Error:/n/n{}", error.to_string())
			}
			RustyTubeError::Websys(error) => write!(f, "Websys Error:/n/n{}", error.to_string()),
			RustyTubeError::Xml(error) => write!(f, "Xml Error:/n/n{}", error.to_string()),
		}
	}
}
