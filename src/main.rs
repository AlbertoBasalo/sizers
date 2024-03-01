// "/home/alber/OneDrive/Documentos/Contenido\\ Cursos/Angular\\ moderno"

extern crate csv;
extern crate mp4;
extern crate serde;
extern crate walkdir;

use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use walkdir::WalkDir;

#[derive(Serialize, Clone, Debug)]
struct MediaFileMetadata {
    folder: String,
    file_name: String,
    size: u64,
    duration: u64,
}

fn main() {
    let command_line_arguments: Vec<String> = env::args().collect();
    if command_line_arguments.len() != 2 {
        println!("🔥 Need a root param.");
        std::process::exit(1);
    }
    let root: &String = &command_line_arguments[1];
    let start = Instant::now();
    println!("🚀 Walking down from Root: {}", root);
    let mut files_metadata: Vec<MediaFileMetadata> = Vec::new();
    let mut csv_writer = csv::Writer::from_path("output.csv").unwrap();
    for entry_result in WalkDir::new(root) {
        let entry = entry_result.unwrap();
        let metadata = get_metadata(&entry);
        if let Some(metadata) = metadata {
            files_metadata.push(metadata.clone());
            csv_writer.serialize(&metadata).unwrap();
        }
    }
    csv_writer.flush().unwrap();
    print_metrics(files_metadata);
    let elapsed = start.elapsed();
    let process_duration = elapsed.as_millis().to_string();
    println!(
        "🦀 File size recollection finished in {} ms. Good bye!",
        process_duration
    );
}

fn get_metadata(entry: &walkdir::DirEntry) -> Option<MediaFileMetadata> {
    let path = entry.path();
    let extension = path.extension().unwrap_or_default();
    if extension == "mp4" {
        let folder = path.parent().unwrap().to_string_lossy();
        let file_name = path.file_name().unwrap().to_string_lossy();
        let file = File::open(&path).unwrap();
        let bytes_length = file.metadata().unwrap().len();
        let duration = get_duration(file, bytes_length);
        let size = bytes_length / 1024 / 1024;
        let metadata = MediaFileMetadata {
            folder: folder.into_owned(),
            file_name: file_name.into_owned(),
            size,
            duration,
        };
        Some(metadata)
    } else {
        None
    }
}

fn get_duration(entry_file: File, entry_bytes_length: u64) -> u64 {
    let entry_reader = BufReader::new(entry_file);
    let mp4_reader = mp4::Mp4Reader::read_header(entry_reader, entry_bytes_length).unwrap();
    let mut duration = 0;
    for track in mp4_reader.tracks().values() {
        duration = track.duration().as_secs() as u64;
    }
    duration
}

fn print_metrics(files_metadata: Vec<MediaFileMetadata>) {
    let total_files: u64 = files_metadata.len() as u64;
    print_metric("Total files", &total_files.to_string());
    let total_duration_seconds: u64 = files_metadata.iter().map(|x| x.duration).sum();
    let total_duration = format_duration(total_duration_seconds);
    print_metric("Total duration", &total_duration);
    let average_duration_seconds = total_duration_seconds / total_files;
    let average_duration = format_duration(average_duration_seconds);
    print_metric("Average duration", &average_duration);

    let min_duration_file = files_metadata.iter().min_by_key(|x| x.duration).unwrap();
    let min_duration_format = format_duration(min_duration_file.duration);
    print_metric(
        &format!("Min duration ({})", min_duration_file.file_name),
        &min_duration_format,
    );

    let max_duration_file = files_metadata.iter().max_by_key(|x| x.duration).unwrap();
    let max_duration_format = format_duration(max_duration_file.duration);
    print_metric(
        &format!("Max duration ({})", max_duration_file.file_name),
        &max_duration_format,
    );
}

// function to format the duration in format hh:mm:ss
fn format_duration(duration: u64) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        duration / 3600,
        (duration % 3600) / 60,
        duration % 60
    )
}

// function to print a metric
fn print_metric(metric: &str, amount: &str) {
    println!("📊 {}: {}", metric, amount);
}
