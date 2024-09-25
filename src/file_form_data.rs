use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub date: HashMap<String, String>,
}

pub fn get_data_form() -> Data{
    // Leer el archivo TOML
    let toml_content = fs::read_to_string("form_data.toml").unwrap();

    // Deserializar el contenido a la estructura Data
    let data: Data = toml::from_str(&toml_content).unwrap();

    data
}
