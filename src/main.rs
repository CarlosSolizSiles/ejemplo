use lopdf::content::{Content, Operation};
use lopdf::{Document, Object};
use std::collections::BTreeMap;

struct ItemText {
    text: &'static str,
    x: f32,
    y: f32,
}

fn add_text(text: &str, x: f32, y: f32) -> Content {
    let content = Content {
        operations: vec![
            // Operación de movimiento a las coordenadas (100, 500)
            Operation::new(
                "Tm",
                vec![2.into(), 0.into(), 0.into(), 2.into(), 0.into(), 0.into()],
            ),
            // Selecciona la fuente estándar (Helvetica) con un tamaño de fuente más pequeño
            Operation::new("Tf", vec!["A1".into(), 5.5.into()]), // Tamaño de la fuente reducido a 5
            Operation::new("Td", vec![x.into(), y.into()]),
            // Operation::new("Td", vec![83.into(), 340.5.into()]),
            // Añade el texto
            Operation::new("Tj", vec![Object::string_literal(text)]),
        ],
    };
    content
}

fn main() {
    // Abre el PDF existente como plantilla
    let mut doc = Document::load("Formato de Entrega PC_v2 6.pdf").unwrap();
    // Obtén el mapa de páginas del documento
    let pages: BTreeMap<u32, lopdf::ObjectId> = doc.get_pages();

    // Obtenemos el ID de la primera página
    let (_page_number, page_id) = pages.iter().next().unwrap(); // Aquí obtenemos la primera página

    let list_item_text = [
        ItemText {
            text: "Soliz Siles Carlos Daniel",
            x: 83.0,
            y: 340.0,
        },
        ItemText {
            text: "PWOCM4Y2",
            x: 83.0,
            y: 333.5,
        },
        ItemText {
            text: "Buenos Aires",
            x: 83.0,
            y: 326.65,
        },
        ItemText {
            text: "24/09/2024",
            x: 172.0,
            y: 340.5,
        },
        ItemText {
            text: "carlos.soliz@siemens.com",
            x: 172.0,
            y: 333.5,
        },
        ItemText {
            text: "Lenovo",
            x: 80.0,
            y: 285.25,
        },
        ItemText {
            text: "thinkpad L14 G4",
            x: 80.0,
            y: 278.0,
        },
        ItemText {
            text: "PWCM0QQQ",
            x: 172.0,
            y: 285.25,
        },
        ItemText {
            text: "32GB",
            x: 172.0,
            y: 278.0,
        },
        ItemText {
            text: "512GB",
            x: 172.0,
            y: 270.65,
        },
    ];

    for item in list_item_text {
        let content = add_text(&item.text, item.x, item.y);
        doc.add_to_page_content(*page_id, content).unwrap();
    }

 // // ! Crea un nuevo contenido para añadir texto
    // let content = add_text("Lenovo", 80.0, 285.5);
    // // Agrega el nuevo contenido a la página existente
    // doc.add_to_page_content(*page_id, content).unwrap();

    // // ! Crea un nuevo contenido para añadir texto
    // let content = add_text("lenovo thinkpad L14 G4", 83.0, 272.5);
    // // Agrega el nuevo contenido a la página existente
    // doc.add_to_page_content(*page_id, content).unwrap();

    // Guarda el archivo PDF modificado
    doc.save("output_modificado.pdf").unwrap();
}
