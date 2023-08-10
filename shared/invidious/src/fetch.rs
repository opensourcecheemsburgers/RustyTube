use std::fmt;
use std::fmt::{Display, Formatter};
use reqwasm::{http::{Method, Request, RequestMode}};
use rustytube_error::RustyTubeError;
use serde::{Deserialize, Serialize};

pub async fn fetch(url: &str) -> Result<String, RustyTubeError> {
    gloo::console::debug!("Fetching url: {}", url);

    let request = Request::new(url).mode(RequestMode::Cors).method(Method::GET);
    Ok(request.send().await?.text().await?)
}
