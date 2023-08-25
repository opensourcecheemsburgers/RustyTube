#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{Config, NetworkConfig};
    use std::env::current_dir;

    #[test]
    fn can_create_default_config() {
        let config = Config::default();
        let toml = config.to_toml_string().unwrap();

        let mut path = current_dir().unwrap();

        path.push("src/tests/files/config.toml");
        fs::write(path, toml).unwrap();
    }

    #[test]
    fn can_create_config_with_custom_servers_and_different_default_server() {
        let config = Config {
            network: NetworkConfig {
                server: "https://iv.nboeck.de".to_string(),
                custom_servers: Some
                    (vec!["https://iv.test.test".to_string(),
                          "https://iv.testicles.testicles".to_string(),
                          "https://iv.bollocks.bollocks".to_string()
                    ]),
                auto_fetch_subs: false,
                fetch_rss: false,
            },
            ui: Default::default(),
            player: Default::default(),
            privacy: Default::default(),
        };
        let toml = config.to_toml_string().unwrap();

        let mut path = current_dir().unwrap();
        path.push("src/tests/files/config.toml");
        fs::write(path, toml).unwrap();
    }

    #[test]
    fn can_read_toml_file() {
        let toml = include_str!("files/config.toml");
        let config: Config = toml::from_str(toml).unwrap();
    }
}