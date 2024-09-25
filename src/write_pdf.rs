use lopdf::content::{Content, Operation};
use lopdf::{Document, Object};
use std::collections::BTreeMap;

use crate::{file_settings, StaticText};

// Función que genera el contenido PDF para insertar texto en una posición específica
fn create_text_content(text: String, coordinates: [f32; 2]) -> Content {
    Content {
        operations: vec![
            // Operación para transformar la posición del texto
            Operation::new(
                "Tm",
                vec![2.into(), 0.into(), 0.into(), 2.into(), 0.into(), 0.into()],
            ),
            // Selección de la fuente Helvetica con un tamaño reducido
            Operation::new("Tf", vec!["A1".into(), 5.5.into()]),
            // Mover el cursor de escritura a las coordenadas especificadas
            Operation::new("Td", vec![coordinates[0].into(), coordinates[1].into()]),
            // Escribir el texto literal en la posición actual
            Operation::new("Tj", vec![Object::string_literal(text)]),
        ],
    }
}

pub fn edit_pdf(text_static: Vec<StaticText>) {
    // Cargar el documento PDF como plantilla
    let mut pdf_document = Document::load("asset/Formato de Entrega.pdf").unwrap();

    // Obtener el mapa de páginas del documento
    let page_map: BTreeMap<u32, lopdf::ObjectId> = pdf_document.get_pages();

    // Obtener el ID de la primera página
    let (_, page_id) = page_map.iter().next().unwrap();

    for item in text_static {
        let content = create_text_content(item.text, item.coordinates);
        pdf_document.add_to_page_content(*page_id, content).unwrap();
    }
    pdf_document.save(format!("{}.pdf", "carlos")).unwrap();
}
