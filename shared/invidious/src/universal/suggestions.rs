use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Suggestions {
    pub query: String,
    pub suggestions: Vec<String>,
}

impl Suggestions {
    fn url(server: &str, args: String) -> String {
        format!("{}/api/v1/search/suggestions/{}", server, args)
    }
}
