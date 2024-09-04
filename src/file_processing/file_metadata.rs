use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use crate::file_processing::file_state::FileState;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileMetadata {
    pub file_path: String,
    pub line_count: usize,
    pub size_bytes: u64,
    pub last_modified: String,
}

pub fn get_all_log_and_txt_files(folder_path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(get_all_log_and_txt_files(&path)?);
        } else if let Some(extension) = path.extension() {
            if extension == "log" || extension == "txt" {
                files.push(path);
            }
        }
    }
    Ok(files)
}

pub fn process_files(file_state: &mut FileState, files_to_check: Vec<PathBuf>, state_file: &str) -> io::Result<()> {
    for path in files_to_check {
        let file_metadata = collect_file_metadata(&path)?;
        let previous_metadata = file_state.get(&file_metadata.file_path);

        if previous_metadata.is_none() || file_metadata.line_count > previous_metadata.unwrap().line_count {
            let new_lines = read_log_file(&file_metadata.file_path)?;

            let previous_line_count = previous_metadata.map_or(0, |meta| meta.line_count);
            if new_lines.len() > previous_line_count {
                let lines_to_print = &new_lines[previous_line_count..];
                for line in lines_to_print {
                    println!("{}|{}", file_metadata.file_path, line);
                }
                file_state.insert(file_metadata);
                crate::file_processing::file_state::save_state(file_state, state_file)?;
            }
        }
    }
    Ok(())
}

fn collect_file_metadata(file_path: &Path) -> io::Result<FileMetadata> {
    let metadata = fs::metadata(file_path)?;
    let file_size = metadata.len();
    let modified_time = metadata.modified()?;
    let last_modified = chrono::DateTime::<chrono::Utc>::from(modified_time).to_rfc3339();

    let line_count = read_log_file(file_path.to_str().unwrap())?.len();

    Ok(FileMetadata {
        file_path: file_path.to_str().unwrap().to_string(),
        line_count,
        size_bytes: file_size,
        last_modified,
    })
}

fn read_log_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    Ok(lines)
}
