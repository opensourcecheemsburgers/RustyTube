use reqwasm::http::{Method, Request, RequestMode};
use rustytube_error::RustyTubeError;
use web_sys::RequestCache;

pub async fn fetch(url: &str) -> Result<String, RustyTubeError> {
	let request = Request::new(url)
		.mode(RequestMode::Cors)
		.method(Method::GET)
		.cache(RequestCache::Default);
	Ok(request.send().await?.text().await?)
}
