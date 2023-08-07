use std::fmt;
use std::fmt::{Display, Formatter};
use crate::fetch::FetchError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RustyTubeError {
    title: String,
    description: String,
}

impl Display for RustyTubeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:/n/n{}", self.title, self.description)
    }
}

impl From<FetchError> for RustyTubeError {
    fn from(fetch_err: FetchError) -> Self {
        let title = fetch_err.title;
        let description = fetch_err.description;
        Self { title, description }
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


