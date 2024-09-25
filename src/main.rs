mod file_form_data;
mod file_settings;
mod write_pdf;

use file_form_data::get_data_form;
use file_settings::get_file_settings_toml;
use serde::Deserialize;
use write_pdf::edit_pdf;

#[derive(Debug, Deserialize)]
struct StaticText {
    text: String,
    coordinates: [f32; 2],
}

fn main() {
    let settings: file_settings::Settings = get_file_settings_toml();
    let data_form = get_data_form();

    let mut insert_text: Vec<StaticText> = Vec::new();

    for text in settings.static_texts {
        if let Some(value) = data_form.date.get(&text.text_id) {
            insert_text.push(StaticText {
                coordinates: text.coordinates,
                text: value.clone(),
            });
        } else {
            insert_text.push(StaticText {
                coordinates: text.coordinates,
                text: "Complete".to_string(),
            });
        }
    }

    edit_pdf(insert_text);
    // println!("{:?}", settings.texto_estaticos);
}
