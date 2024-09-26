use lopdf::content::{Content, Operation};
use lopdf::{Document, Object};
use std::collections::BTreeMap;

use crate::StaticText;

// Función que genera el contenido PDF para insertar texto en una posición específica
fn create_text_content(text: String, coordinates: [f32; 2]) -> Content {
    Content {
        operations: vec![
            // BT comienza un elemento de texto
            Operation::new("BT", vec![]),
            // Tf especifica la fuente y el tamaño
            Operation::new("Tf", vec!["A1".into(), 11.0.into()]),
            // Td ajusta la posición del texto
            Operation::new("Td", vec![coordinates[0].into(), coordinates[1].into()]),
            // Tj inserta el texto literal
            Operation::new("Tj", vec![Object::string_literal(text)]),
            // ET termina el elemento de texto
            Operation::new("ET", vec![]),
        ],
    }
}

pub fn modify_pdf_content(text_static: Vec<StaticText>, name_file: String) {
    // Cargar el documento PDF como plantilla
    let mut pdf_document = Document::load("asset/Formato de Entrega.pdf").unwrap();
    pdf_document.version = "1.7".to_string();

    // Obtener el mapa de páginas del documento
    let page_map: BTreeMap<u32, lopdf::ObjectId> = pdf_document.get_pages();

    // Obtener el ID de la primera página
    let (_, page_id) = page_map.iter().next().unwrap();

    for item in text_static {
        // Crear el contenido del texto usando las coordenadas y el texto proporcionado
        let content = create_text_content(item.content, item.position);
        pdf_document.add_to_page_content(*page_id, content).unwrap();
    }
    pdf_document.save(format!("{}.pdf", name_file)).unwrap();
}
