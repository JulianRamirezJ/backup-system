use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BackupInfo {
    pub(crate) input_folder: String,
    pub(crate) output_file: String,
    pub(crate) chunk_size: usize,
    pub(crate) split_files: Vec<String>,
    pub(crate) pass: String,
    pub(crate) key: Vec<u8>
}