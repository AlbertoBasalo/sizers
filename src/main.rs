// "/home/alber/OneDrive/Documentos/Contenido\\ Cursos/Angular\\ moderno"

extern crate csv;
extern crate mp4;
extern crate serde;
extern crate walkdir;

use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::BufReader;
use walkdir::WalkDir;

#[derive(Serialize, Clone, Debug)]
struct MediaFileMetadata {
    // full_path: String,
    folder: String,
    file_name: String,
    size: u64,
    duration: u64,
    minutes: u64,
    seconds: u64,
}
//let root = "/c/Users/alber/OneDrive/Documentos/VideoCursos/";

fn main() {
    let mut csv_writer = csv::Writer::from_path("output.csv").unwrap();
    let args: Vec<String> = env::args().collect();
    let root: &String = &args[1];
    // vector to store the file metadata
    let mut files_metadata: Vec<MediaFileMetadata> = Vec::new();
    println!("Root: {}", root);
    for entry_result in WalkDir::new(root) {
        let entry = entry_result.unwrap();
        let path = entry.path();
        if path.extension().unwrap_or_default() == "mp4" {
            let file = File::open(&path).unwrap();
            let bytes = file.metadata().unwrap().len();
            // let full_path = path.to_string_lossy();
            let folder = path.parent().unwrap().to_string_lossy();
            let file_name = path.file_name().unwrap().to_string_lossy();
            let reader = BufReader::new(file);
            let mp4_reader = mp4::Mp4Reader::read_header(reader, bytes).unwrap();
            let mut duration = 0;
            for track in mp4_reader.tracks().values() {
                duration = track.duration().as_secs() as u64;
            }
            let size = bytes / 1024 / 1024;
            // println!("{} -> size: {} - duration: {}", file_name, size, duration);
            let metadata = MediaFileMetadata {
                // full_path: full_path.into_owned(),
                folder: folder.into_owned(),
                file_name: file_name.into_owned(),
                size,
                duration,
                minutes: duration / 60,
                seconds: duration % 60,
            };
            // store the metadata in the vector
            files_metadata.push(metadata.clone());
            csv_writer.serialize(metadata).unwrap();
        }
    }
    csv_writer.flush().unwrap();
    // get max duration
    let max_duration_file = files_metadata.iter().max_by_key(|x| x.duration).unwrap();
    // get min duration
    let min_duration_file = files_metadata.iter().min_by_key(|x| x.duration).unwrap();
    // get average duration
    let total_duration_seconds: u64 = files_metadata.iter().map(|x| x.duration).sum();
    let average_duration_seconds = total_duration_seconds / files_metadata.len() as u64;
    // print total duration in format hh:mm:ss
    let total_duration = format!(
        "{:02}:{:02}:{:02}",
        total_duration_seconds / 3600,
        (total_duration_seconds % 3600) / 60,
        total_duration_seconds % 60
    );
    println!("Total duration: {}", total_duration);
    // print average duration in format hh:mm:ss
    let average_duration = format!(
        "{:02}:{:02}:{:02}",
        average_duration_seconds / 3600,
        (average_duration_seconds % 3600) / 60,
        average_duration_seconds % 60
    );
    println!("Average duration: {}", average_duration);
    // print min and max duration in format hh:mm:ss
    let min_duration_format = format!(
        "{:02}:{:02}:{:02}",
        min_duration_file.duration / 3600,
        (min_duration_file.duration % 3600) / 60,
        min_duration_file.duration % 60
    );
    println!(
        "Min duration: {} - {}",
        min_duration_file.file_name, min_duration_format
    );
    let max_duration_format = format!(
        "{:02}:{:02}:{:02}",
        max_duration_file.duration / 3600,
        (max_duration_file.duration % 3600) / 60,
        max_duration_file.duration % 60
    );
    println!(
        "Max duration: {} - {}",
        max_duration_file.file_name, max_duration_format
    );
}
