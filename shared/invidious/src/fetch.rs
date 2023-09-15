use std::fmt::Display;
use reqwasm::{http::{Method, Request, RequestMode}};
use web_sys::RequestCache;
use rustytube_error::RustyTubeError;

pub async fn fetch(url: &str) -> Result<String, RustyTubeError> {
    let request = Request::new(&url).mode(RequestMode::Cors).method(Method::GET).cache(RequestCache::Default);
    Ok(request.send().await?.text().await?)
}