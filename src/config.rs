use anyhow::Result;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE: &str = "config.ron";
const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "rustylock";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub color_scheme: ColorScheme,
    pub password_options: PasswordOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColorScheme {
    Dark,
    Light,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordOptions {
    pub length: u8,
    pub use_symbols: bool,
    pub use_spaces: bool,
    pub use_numbers: bool,
    pub use_uppercase: bool,
    pub use_lowercase: bool,
    pub exclude_similar_characters: bool,
}

struct ConfigPath {
    config_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Config {
            ..Default::default()
        }
    }

    fn get_or_build_config() -> Result<ConfigPath> {
        match dirs::home_dir() {
            Some(path) => {
                let config_path = path.join(CONFIG_DIR).join(APP_CONFIG_DIR);

                if !config_path.exists() {
                    std::fs::create_dir_all(&config_path)?;
                }

                let config_file_path = config_path.join(CONFIG_FILE);

                Ok(ConfigPath {
                    config_path: config_file_path,
                })
            }
            None => {
                eprintln!("Error getting home directory");
                std::process::exit(1);
            }
        }
    }

    pub fn load_config(&self) -> Self {
        let config_path = Config::get_or_build_config().unwrap().config_path;

        if config_path.exists() {
            let config_file = std::fs::read_to_string(config_path).unwrap();
            let config: Config = ron::from_str(&config_file).unwrap();

            config
        } else {
            let pretty = PrettyConfig::new()
                .depth_limit(2)
                .separate_tuple_members(true)
                .enumerate_arrays(true)
                .extensions(ron::extensions::Extensions::IMPLICIT_SOME);

            let s = to_string_pretty::<Config>(&Default::default(), pretty).unwrap();

            std::fs::write(config_path, s).unwrap();

            Self::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            color_scheme: ColorScheme::Dark,
            password_options: PasswordOptions {
                length: 32,
                use_symbols: true,
                use_spaces: false,
                use_numbers: true,
                use_uppercase: true,
                use_lowercase: true,
                exclude_similar_characters: true,
            },
        }
    }
}
