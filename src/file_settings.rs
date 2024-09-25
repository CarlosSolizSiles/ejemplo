use serde::Deserialize;
use std::{fs::read_to_string, fs::File, io::Read};


#[derive(Debug, Deserialize)]
pub struct StaticText {
    pub text_id: String,
    pub coordinates: [f32; 2],
}

#[derive(Debug, Deserialize)]
pub struct DynamicText {
    pub text_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub static_texts: Vec<StaticText>,
    pub dynamic_text: Vec<DynamicText>,
}

pub fn get_file_settings_toml() -> Settings {
    let toml_content = read_to_string("asset/settings.toml").unwrap();
    let texts: Settings = toml::from_str(&toml_content).unwrap();
    texts
}

pub fn get_file_settings_json() -> Settings {
    let mut file = File::open("asset/settings.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let settings: Settings = serde_json::from_str(&data).expect("JSON was not well-formatted");
    settings
}