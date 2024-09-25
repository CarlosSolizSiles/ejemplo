use lopdf::content::{Content, Operation};
use lopdf::{Document, Object};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Estructura que representa un elemento de texto y su posición en el PDF
struct TextElement {
    text: String,
    position_x: f32,
    position_y: f32,
}

// Función que genera el contenido PDF para insertar texto en una posición específica
fn create_text_content(text: String, position_x: f32, position_y: f32) -> Content {
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
            Operation::new("Td", vec![position_x.into(), position_y.into()]),
            // Escribir el texto literal en la posición actual
            Operation::new("Tj", vec![Object::string_literal(text)]),
        ],
    }
}

fn main() -> io::Result<()> {
    // Abre el archivo
    let file = File::open("data.txt")?;

    // Usa BufReader para leerlo línea por línea
    let reader = BufReader::new(file);

    // Crea un vector para almacenar las líneas
    let lines: Vec<String> = reader
        .lines() // Obtén un iterador sobre las líneas
        .collect::<Result<Vec<String>, _>>()?; // Colecta las líneas en un vector y maneja posibles errores

    // Cargar el documento PDF como plantilla
    let mut pdf_document = Document::load("asset/Formato de Entrega PC_v2 6.pdf").unwrap();
    // Obtener las páginas del documento
    let page_map: BTreeMap<u32, lopdf::ObjectId> = pdf_document.get_pages();

    // Obtener el ID de la primera página
    let (_page_number, page_id) = page_map.iter().next().unwrap();

    let binding = lines[5].clone();
    let reason_code = binding.as_str(); // Código que determina la razón de selección

    // Definir el marcador para la razón seleccionada
    let reason_marker = match reason_code {
        "Instalación Nueva" => TextElement {
            text: "X".to_string(),
            position_x: 73.5,
            position_y: 298.65,
        },
        "Reasignación" => TextElement {
            text: "X".to_string(),
            position_x: 135.5,
            position_y: 298.65,
        },
        "Baja" => TextElement {
            text: "X".to_string(),
            position_x: 203.5,
            position_y: 298.65,
        },
        _ => TextElement {
            text: "".to_string(),
            position_x: 0.0,
            position_y: 0.0,
        },
    };

    // Lista de elementos de texto con sus posiciones correspondientes
    let text_elements = [
        TextElement {
            text: lines[0].clone(),
            position_x: 83.0,
            position_y: 340.0,
        },
        TextElement {
            text: lines[1].clone(),
            position_x: 83.0,
            position_y: 333.5,
        },
        TextElement {
            text: lines[2].clone(),
            position_x: 83.0,
            position_y: 326.65,
        },
        TextElement {
            text: lines[3].clone(),
            position_x: 172.0,
            position_y: 340.5,
        },
        TextElement {
            text: lines[4].clone(),
            position_x: 172.0,
            position_y: 333.5,
        },
        TextElement {
            text: lines[6].clone(),
            position_x: 80.0,
            position_y: 285.25,
        },
        TextElement {
            text: lines[7].clone(),
            position_x: 80.0,
            position_y: 278.0,
        },
        TextElement {
            text: lines[8].clone(),
            position_x: 172.0,
            position_y: 285.25,
        },
        TextElement {
            text: lines[9].clone(),
            position_x: 172.0,
            position_y: 278.0,
        },
        TextElement {
            text: lines[10].clone(),
            position_x: 172.0,
            position_y: 270.65,
        },
        reason_marker, // Añadir el marcador de razón seleccionado
    ];

    // Agregar cada elemento de texto al contenido de la página del PDF
    for element in text_elements {
        let content = create_text_content(element.text, element.position_x, element.position_y);
        pdf_document.add_to_page_content(*page_id, content).unwrap();
    }

    // Guardar el archivo PDF modificado
    pdf_document.save("output_modified.pdf").unwrap();
    Ok(())
}
