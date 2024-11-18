use flate2::read::GzDecoder;
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use csv::Writer;
use serde::{Deserialize, Serialize};
use rayon::ThreadPoolBuilder;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize)]
struct MafRecord {
    #[serde(rename = "Hugo_Symbol")]
    hugo_symbol: String,
    #[serde(rename = "Chromosome")]
    chromosome: String,
    #[serde(rename = "Start_Position")]
    start_position: String,
    #[serde(rename = "End_Position")]
    end_position: String,
    #[serde(rename = "Variant_Classification")]
    variant_classification: String,
    #[serde(rename = "Variant_Type")]
    variant_type: String,
    #[serde(rename = "Tumor_Sample_Barcode")]
    tumor_sample_barcode: String,
    #[serde(rename = "t_depth")]
    t_depth: String,
    #[serde(rename = "t_ref_count")]
    t_ref_count: String,
    #[serde(rename = "t_alt_count")]
    t_alt_count: String,
    #[serde(rename = "Consequence")]
    consequence: String,
    #[serde(rename = "IMPACT")]
    impact: String,
}

fn process_maf_file(input_path: &Path, writer: &mut Writer<File>) -> io::Result<()> {
    // Descomprimir el archivo .gz
    let file = File::open(input_path)?;
    let decoder = GzDecoder::new(file);
    let buffered = io::BufReader::new(decoder);

    // Leer el archivo MAF
    for line in buffered.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;  // Saltar líneas de encabezado
        }

        // Dividir la línea por tabuladores y convertirla a MafRecord
        let columns: Vec<&str> = line.split('\t').collect();
        if columns.len() > 11 {
            let record = MafRecord {
                hugo_symbol: columns[0].to_string(),
                chromosome: columns[4].to_string(),
                start_position: columns[5].to_string(),
                end_position: columns[6].to_string(),
                variant_classification: columns[8].to_string(),
                variant_type: columns[9].to_string(),
                tumor_sample_barcode: columns[15].to_string(),
                t_depth: columns[39].to_string(),
                t_ref_count: columns[40].to_string(),
                t_alt_count: columns[41].to_string(),
                consequence: columns[105].to_string(),
                impact: columns[106].to_string(),
            };
            writer.serialize(record)?;
        }
    }

    writer.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    // Configurar el número de hilos
    let num_threads = 6; // Puedes ajustar este valor según la cantidad de hilos que deseas utilizar
    ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();

    // Ruta al directorio "u3/data"
    let current_folder = std::env::current_dir()?;
    let data_folder = current_folder.join("../../u3/data");
    let combined_output_path = current_folder.join("filtered_maf_combined.csv");

    // Crear el archivo combinado final con un Mutex para proteger el acceso concurrente
    let combined_writer = Arc::new(Mutex::new(Writer::from_path(&combined_output_path)?));

    // Escribir encabezado al archivo CSV combinado
    {
        let mut writer = combined_writer.lock().unwrap();
        writer.write_record(&[
            "Hugo_Symbol", "Chromosome", "Start_Position", "End_Position",
            "Variant_Classification", "Variant_Type", "Tumor_Sample_Barcode",
            "t_depth", "t_ref_count", "t_alt_count", "Consequence", "IMPACT",
        ])?;
    }

    // Obtener la lista de carpetas UUID dentro de la carpeta "data"
    let entries = fs::read_dir(data_folder)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    // Procesar en lotes para evitar el uso excesivo de memoria
    let batch_size = 100;
    for batch in entries.chunks(batch_size) {
        batch.par_iter().for_each(|entry| {
            let uuid_folder = entry.path();
            if let Ok(maf_files) = fs::read_dir(&uuid_folder) {
                for maf_entry in maf_files.filter_map(Result::ok) {
                    let maf_path = maf_entry.path();
                    if maf_path.extension().and_then(|ext| ext.to_str()) == Some("gz") {
                        let mut writer = match combined_writer.lock() {
                            Ok(writer) => writer,
                            Err(err) => {
                                eprintln!("Error obteniendo el bloqueo del archivo combinado: {}", err);
                                return;
                            }
                        };
                        if let Err(err) = process_maf_file(&maf_path, &mut writer) {
                            eprintln!("Error procesando {:?}: {}", maf_path, err);
                        }
                    }
                }
            }
        });
    }

    println!("Procesamiento completo.");

    Ok(())
}
