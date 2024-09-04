use serde::{Serialize, Deserialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use crate::utils::generate_metadata;
use crate::file_processing::file_metadata::FileMetadata;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileState {
    pub metadata: crate::utils::Metadata,
    pub log_files: Vec<FileMetadata>,
    pub txt_files: Vec<FileMetadata>,
}

impl FileState {
    pub fn new() -> Self {
        FileState {
            metadata: generate_metadata(),
            log_files: Vec::new(),
            txt_files: Vec::new(),
        }
    }

    pub fn insert(&mut self, file_metadata: FileMetadata) {
        let file_list = if file_metadata.file_path.ends_with(".log") {
            &mut self.log_files
        } else if file_metadata.file_path.ends_with(".txt") {
            &mut self.txt_files
        } else {
            return;
        };

        if let Some(existing_file) = file_list.iter_mut().find(|f| f.file_path == file_metadata.file_path) {
            *existing_file = file_metadata;
        } else {
            file_list.push(file_metadata);
        }
    }

    pub fn get(&self, file_path: &str) -> Option<&FileMetadata> {
        if file_path.ends_with(".log") {
            self.log_files.iter().find(|&f| f.file_path == file_path)
        } else if file_path.ends_with(".txt") {
            self.txt_files.iter().find(|&f| f.file_path == file_path)
        } else {
            None
        }
    }
}

pub fn load_state(state_file: &str) -> io::Result<FileState> {
    if Path::new(state_file).exists() {
        let state_json = fs::read_to_string(state_file)?;
        let state: FileState = serde_json::from_str(&state_json)?;
        Ok(state)
    } else {
        Ok(FileState::new())
    }
}

pub fn save_state(file_state: &FileState, state_file: &str) -> io::Result<()> {
    let state_json = serde_json::to_string_pretty(file_state)?;
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(state_file)?;
    file.write_all(state_json.as_bytes())?;
    Ok(())
}
