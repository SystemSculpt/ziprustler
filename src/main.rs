use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let directory = determine_directory(&args)?;

    let files = traverse_directory(&directory)?;
    let backup_name = format_backup_name(&directory)?;
    let backup_path = expand_home_directory(&backup_name)?;
    create_backup(&directory, &files, &backup_path)
}

fn determine_directory(args: &[String]) -> io::Result<String> {
    Ok(args
        .get(1)
        .cloned()
        .unwrap_or_else(|| env::current_dir().unwrap().to_str().unwrap().to_string()))
}

fn traverse_directory(directory: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    traverse_recursive(directory, &mut files, directory)?;
    Ok(files)
}

fn traverse_recursive(
    base_directory: &str,
    files: &mut Vec<String>,
    directory: &str,
) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path
            .strip_prefix(base_directory)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        if path.is_dir() {
            traverse_recursive(base_directory, files, path.to_str().unwrap())?;
        } else {
            files.push(relative_path);
        }
    }
    Ok(())
}

fn format_backup_name(directory: &str) -> io::Result<String> {
    let timestamp = Local::now().format("%Y%m%d_%H%M").to_string();
    let dir_name = Path::new(directory).file_name().unwrap().to_str().unwrap();
    Ok(format!("~/.rustler_backups/{}_{}.zip", dir_name, timestamp))
}

fn expand_home_directory(path: &str) -> io::Result<PathBuf> {
    let mut path = PathBuf::from(path);
    if path.starts_with("~") {
        let home_dir = env::var("HOME").unwrap();
        path = Path::new(&home_dir).join(path.strip_prefix("~").unwrap());
    }
    Ok(path)
}

fn create_backup(base_directory: &str, files: &[String], backup_path: &PathBuf) -> io::Result<()> {
    let total_size = files
        .iter()
        .map(|path| {
            fs::metadata(Path::new(base_directory).join(path))
                .unwrap()
                .len()
        })
        .sum();

    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    fs::create_dir_all(backup_path.parent().unwrap())?;
    let file = File::create(backup_path)?;
    let mut zip = ZipWriter::new(file);

    for relative_path in files {
        let absolute_path = Path::new(base_directory).join(relative_path);
        let metadata = fs::metadata(&absolute_path)?;
        let file_size = metadata.len();

        zip.start_file(
            relative_path,
            FileOptions::default().compression_method(CompressionMethod::Deflated),
        )?;

        let mut file = File::open(absolute_path)?;
        io::copy(&mut file, &mut zip)?;

        progress_bar.inc(file_size);
    }

    zip.finish()?;
    progress_bar.finish_with_message("Compression complete");

    Ok(())
}
