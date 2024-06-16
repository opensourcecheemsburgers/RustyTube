use invidious::{fetch_instance_info, Instances};
use leptos::Resource;
use rustytube_error::RustyTubeError;

use super::save_resource;

static INSTANCES_KEY: &str = "instances";

#[derive(Clone, Copy)]
pub struct InstancesResource {
	pub resource: Resource<(), Result<Instances, RustyTubeError>>,
}

impl InstancesResource {
	pub fn initialise() -> Self {
		let resource = Resource::local(move || (), move |()| fetch_instances());
		Self { resource }
	}
}

async fn fetch_instances() -> Result<Instances, RustyTubeError> {
	let instances = fetch_instance_info().await?;
	save_resource(INSTANCES_KEY, &instances).await?;
	Ok(instances)
}
