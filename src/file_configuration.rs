use serde::Deserialize;
use std::{fs::read_to_string, fs::File, io::Read};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub name_file: String,
}

#[derive(Debug, Deserialize)]
pub struct StaticText {
    pub label_id: String,
    pub position: [f32; 2],
}

#[derive(Debug, Deserialize)]
pub struct DynamicText {
    pub group_id: String,
    pub label_id: String,
    pub position: [f32; 2],
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub settings: Settings,
    pub static_texts: Vec<StaticText>,
    pub dynamic_text: Vec<DynamicText>,
}

pub fn get_file_configuration_toml() -> Configuration {
    let toml_content = read_to_string("asset/settings.toml").unwrap();
    let texts: Configuration = toml::from_str(&toml_content).unwrap();
    texts
}

pub fn _get_file_settings_json() -> Configuration {
    let mut file = File::open("asset/settings.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let settings: Configuration = serde_json::from_str(&data).expect("JSON was not well-formatted");
    settings
}