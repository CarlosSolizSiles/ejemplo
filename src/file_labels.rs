use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct FormOptions {
    pub option: String,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub date: HashMap<String, String>,
    pub options: Vec<FormOptions>
}

pub fn get_labels() -> Data{
    // Leer el archivo TOML
    let toml_content = fs::read_to_string("labels.toml").unwrap();

    // Deserializar el contenido a la estructura Data
    let data: Data = toml::from_str(&toml_content).unwrap();

    data
}
