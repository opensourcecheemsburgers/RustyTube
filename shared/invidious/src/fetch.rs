use std::fmt::Display;
use reqwasm::{http::{Method, Request, RequestMode}};
use rustytube_error::RustyTubeError;

pub async fn fetch(url: &str) -> Result<String, RustyTubeError> {
    let request = Request::new(url).mode(RequestMode::Cors).method(Method::GET);
    Ok(request.send().await?.text().await?)
}
