use chrono::ParseError;
use gloo::file::FileReadError;
use gloo::storage::errors::StorageError;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug, Display, Formatter},
    io,
    ops::Range,
    string::FromUtf8Error,
};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RustyTubeError {
    pub title: String,
    pub description: String,
}

impl Display for RustyTubeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:/n/n{}", self.title, self.description)
    }
}

impl From<serde_json::Error> for RustyTubeError {
    fn from(serde_json_err: serde_json::Error) -> Self {
        let title = String::from("Serde Error");
        let description = serde_json_err.to_string();
        Self { title, description }
    }
}

impl From<csv::Error> for RustyTubeError {
    fn from(csv_err: csv::Error) -> Self {
        let title = String::from("CSV Error");
        let description = csv_err.to_string();
        Self { title, description }
    }
}

impl From<io::Error> for RustyTubeError {
    fn from(io_error: io::Error) -> Self {
        let title = String::from("I/O Error");
        let description = io_error.kind().to_string();
        Self { title, description }
    }
}

impl From<tauri_sys::error::Error> for RustyTubeError {
    fn from(tauri_error: tauri_sys::error::Error) -> Self {
        let mut tauri_error_object_string = tauri_error.to_string();

        let range = Range { start: 0, end: 27 };
        tauri_error_object_string.replace_range(range, "");
        tauri_error_object_string.pop();
        tauri_error_object_string.pop();

        // Example tauri-sys error.
        //
        // "JS Binding: JsValue(Object({"description":"There was no save path selected.","title":"Save Error","verbose_description":null}))"

        let error: RustyTubeError = serde_json::from_str(&tauri_error_object_string).unwrap();
        error
    }
}

impl From<ron::Error> for RustyTubeError {
    fn from(ron_error: ron::Error) -> Self {
        let title = String::from("RON Error");
        let description = ron_error.to_string();
        Self { title, description }
    }
}

impl From<FromUtf8Error> for RustyTubeError {
    fn from(utf8_error: FromUtf8Error) -> Self {
        let title = String::from("File Read Error");
        let description = utf8_error.to_string();
        Self { title, description }
    }
}

impl From<ron::error::SpannedError> for RustyTubeError {
    fn from(ron_error: ron::error::SpannedError) -> Self {
        let title = String::from("RON Error");
        let description = ron_error.to_string();
        Self { title, description }
    }
}

impl From<StorageError> for RustyTubeError {
    fn from(storage_error: StorageError) -> Self {
        let title = String::from("Browser Storage Error");
        let description = storage_error.to_string();
        Self { title, description }
    }
}

impl From<reqwasm::Error> for RustyTubeError {
    fn from(reqwasm_error: reqwasm::Error) -> Self {
        let title = String::from("Network Error");
        let description = reqwasm_error.to_string();
        Self { title, description }
    }
}

impl From<FileReadError> for RustyTubeError {
    fn from(file_read_error: FileReadError) -> Self {
        let title = String::from("File Read Error");
        let description = file_read_error.to_string();
        Self { title, description }
    }
}

impl From<toml::ser::Error> for RustyTubeError {
    fn from(toml_ser_error: toml::ser::Error) -> Self {
        let title = String::from("Toml Serialisation Error");
        let description = toml_ser_error.to_string();
        Self { title, description }
    }
}

impl From<toml::de::Error> for RustyTubeError {
    fn from(toml_de_error: toml::de::Error) -> Self {
        let title = String::from("Toml Serialisation Error");
        let description = toml_de_error.to_string();
        Self { title, description }
    }
}

impl From<ParseError> for RustyTubeError {
    fn from(parse_error: ParseError) -> Self {
        let title = String::from("Chrono RFC3339 parse error");
        let description = parse_error.to_string();
        Self { title, description }
    }
}

impl From<serde_xml_rs::Error> for RustyTubeError {
    fn from(xml_error: serde_xml_rs::Error) -> Self {
        let title = String::from("XML parse error");
        let description = xml_error.to_string();
        Self { title, description }
    }
}

impl From<JsValue> for RustyTubeError {
    fn from(js_value: JsValue) -> Self {
        let title = String::from("Error");
        let description = js_value.as_string().unwrap_or_default();
        Self { title, description }
    }
}

impl RustyTubeError {
    pub fn from(title: String, description: String) -> Self {
        Self { title, description }
    }

    pub fn element_not_found(id: &str) -> Self {
        let title = String::from("Element Error");
        let description = format!(
            "An element with id: '{}' could not be found in the window.",
            id
        );
        Self { title, description }
    }

    pub fn dyn_into_fail(id: &str) -> Self {
        let title = String::from("Element Error");
        let description = format!(
            "An element with id: '{}' could not be dynamically changed.",
            id
        );
        Self { title, description }
    }

    pub fn fetch_thumbnail_error() -> Self {
        let title = String::from("Network Error");
        let description = String::from("Could not fetch thumbnail.");
        Self { title, description }
    }

    pub fn no_file_selected() -> Self {
        let title = String::from("I/O Error");
        let description = String::from("No file was chosen in the file dialog.");
        Self { title, description }
    }

    pub fn parse_playlist_fail() -> Self {
        let title = String::from("Playlist Error");
        let description = String::from("Unable to parse playlist(s) from chosen file.");
        Self { title, description }
    }

    pub fn format_parse() -> Self {
        let title = String::from("Format Parse Error");
        let description = String::from("Unable to parse format.");
        Self { title, description }
    }

    pub fn no_dash_video_format_available() -> Self {
        let title = String::from("DASH error");
        let description = String::from("No DASH video format available.");
        Self { title, description }
    }

    pub fn format_not_available() -> Self {
        let title = String::from("Format Error");
        let description = String::from("There is no format available.");
        Self { title, description }
    }

    pub fn no_legacy_format_available() -> Self {
        let title = String::from("Legacy Format Error");
        let description = String::from("No legacy video format available.");
        Self { title, description }
    }

    pub fn no_audio_format_available() -> Self {
        let title = String::from("Audio Format error");
        let description = String::from("No audio format available.");
        Self { title, description }
    }

    pub fn no_container_info(name: &str) -> Self {
        let title = String::from("Container Info Error");
        let description = format!("Unable to parse container info on format: {}", name);
        Self { title, description }
    }

    pub fn search_url_parse() -> Self {
        let title = String::from("Search Url Parse Error");
        let description = format!("Unable to parse search url.");
        Self { title, description }
    }

    pub fn no_video_url_avaiable() -> Self {
        let title = String::from("Video Url Error");
        let description = format!("Could not find a video url on the selected format.");
        Self { title, description }
    }

    pub fn no_audio_url_avaiable() -> Self {
        let title = String::from("Audio Url Error");
        let description = format!("Could not find an audio url on the selected format.");
        Self { title, description }
    }
}

