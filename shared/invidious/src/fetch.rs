use std::fmt;
use std::fmt::{Display, Formatter};
use reqwasm::{http::{Method, Request, RequestMode}};
use serde::{Deserialize, Serialize};

pub async fn fetch(url: &str) -> Result<String, FetchError> {
    gloo::console::debug!("Fetching url: {}", url);

    let request = Request::new(url).mode(RequestMode::Cors).method(Method::GET);
    Ok(request.send().await?.text().await?)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FetchError {
    pub title: String,
    pub description: String,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:/n/n{}", self.title, self.description)
    }
}

impl From<reqwasm::Error> for FetchError {
    fn from(reqwasm_err: reqwasm::Error) -> Self {
        let title = String::from("Network Request Error");
        let description = reqwasm_err.to_string();
        Self { title, description }
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(serde_json_err: serde_json::Error) -> Self {
        let title = String::from("Network Request Error");
        let description = serde_json_err.to_string();
        Self { title, description }
    }
}




