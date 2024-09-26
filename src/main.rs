mod file_configuration;
mod file_labels;
mod modify_pdf;

use std::collections::HashMap;

use file_configuration::get_file_configuration_toml;
use file_labels::get_labels;
use modify_pdf::modify_pdf_content;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StaticText {
    content: String,
    position: [f32; 2],
}

fn main() {
    let config: file_configuration::Configuration = get_file_configuration_toml();
    let label_list: file_labels::Data = get_labels();

    let mut text_to_insert: Vec<StaticText> = Vec::new();
    let mut options_to_insert: HashMap<String, HashMap<String, StaticText>> = HashMap::new();

    for dynamic_text in config.dynamic_text {
        if let Some(value) = options_to_insert.get_mut(&dynamic_text.group_id) {
            value.insert(
                dynamic_text.label_id,
                StaticText {
                    content: "X".to_string(),
                    position: dynamic_text.position,
                },
            );
        } else {
            let mut map: HashMap<String, StaticText> = HashMap::new();
            map.insert(
                dynamic_text.label_id,
                StaticText {
                    content: "X".to_string(),
                    position: dynamic_text.position,
                },
            );
            options_to_insert.insert(dynamic_text.group_id, map);
        }
    }

    for static_text in config.static_texts {
        if let Some(value) = label_list.date.get(&static_text.label_id) {
            text_to_insert.push(StaticText {
                position: static_text.position,
                content: value.clone(),
            });
        }
    }

    for item in label_list.options {
        for (_, value) in &options_to_insert {
            if let Some(value) = value.get(&item.option) {
                text_to_insert.push(StaticText {
                    content: value.content.clone(),
                    position: value.position,
                });
            }
        }
    }

    let name_file = if let Some(value) = label_list.date.get(&config.settings.name_file) {
        value.clone()
    } else {
        "".to_string()
    };

    modify_pdf_content(text_to_insert, name_file);
}
