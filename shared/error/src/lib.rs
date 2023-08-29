use std::{fmt::{Formatter, Display, self, Debug}, io, string::FromUtf8Error, ops::Range};
use gloo::file::FileReadError;
use serde::{Serialize, Deserialize};
use gloo::storage::errors::StorageError;
use chrono::ParseError;

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

impl RustyTubeError {
    pub fn from(title: String, description: String) -> Self {        
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

    pub fn no_dash_format_available() -> Self {
        let title = String::from("Playlist Error");
        let description = String::from("Unable to parse playlist(s) from chosen file.");
        Self { title, description }
    }

    pub fn no_container_info(name: &str) -> Self {
        let title = String::from("Container Info Error");
        let description = format!("Unable to parse container info on format: {}", name);
        Self { title, description }
    }
}