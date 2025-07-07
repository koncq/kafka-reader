use crate::profile::ProfileData;
use config::{Config, FileFormat};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;

pub fn get_profiles() -> Result<ProfilesConfig, config::ConfigError> {
    let settings = get_configuration()?;
    settings.try_deserialize::<ProfilesConfig>()
}

pub fn save_configuration(config: &ProfilesConfig) -> color_eyre::Result<(), Error> {
    write_config_to_file(config, &get_home_dir(String::from("settings.toml")))
}

fn get_configuration() -> Result<Config, config::ConfigError> {
    let home_dir = &get_home_dir(String::from("settings.toml"));
    let config_result = read_config_from_file(&home_dir);
    match &config_result {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            write_config_to_file(&ProfilesConfig { profiles: vec![] }, &home_dir).unwrap()
        }
        Err(e) => {
            panic!("Error reading config file: {:?}", e)
        }
        _ => (),
    }

    let settings = Config::builder()
        .add_source(config::File::new(&home_dir, FileFormat::Toml))
        .build()?;

    Ok(settings)
}

fn read_config_from_file(path: &str) -> color_eyre::Result<ProfilesConfig, Error> {
    let data = fs::read(path)?;
    let text = String::from_utf8(data).unwrap();
    let config: ProfilesConfig = toml::from_str(&text).unwrap();
    Ok(config)
}

fn write_config_to_file(config: &ProfilesConfig, path: &str) -> color_eyre::Result<(), Error> {
    let text = toml::to_string(config).unwrap();
    fs::write(path, text)?;
    Ok(())
}

fn get_home_dir(file_name: String) -> String {
    let mut home_dir_path = std::env::home_dir().unwrap();
    home_dir_path.push(&file_name);
    let home_dir = home_dir_path.to_str().unwrap().to_string();
    home_dir
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfilesConfig {
    pub profiles: Vec<ProfileData>,
}
