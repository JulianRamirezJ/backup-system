use std::fs;
use std::io;
use std::fs::File;
use std::error::Error;
use tar::{Builder,Archive};
use std::path::Path;
use std::result::Result;

pub fn create_tarball(input_folder: &String, output_folder: &String) -> io::Result<String> {
    let last_folder_name = Path::new(input_folder)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let output_file = format!("{}/{}/{}.tar", output_folder, last_folder_name, last_folder_name);
    let compressed_folder = format!("{}/{}", output_folder, last_folder_name);
    let output_dir = Path::new(&compressed_folder);

    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Folder Already Backuped"));
    }

    let tar_file = File::create(&output_file)?;
    let mut builder = Builder::new(tar_file);
    let root_path = Path::new(input_folder);
    builder.append_dir_all("", root_path)?;

    Ok(output_file)
}


pub fn restore_from_tarball(input_folder: &String, output_folder: &String) -> Result<(), Box<dyn Error>> {
    let last_folder_name = Path::new(input_folder)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

    let input_file = format!("{}/{}.{}", input_folder, last_folder_name, "tar");
    let output_dir = format!("{}/{}", output_folder, last_folder_name);
    fs::create_dir_all(output_dir.clone())?;
    let tar = std::fs::File::open(input_file.clone())?;
    let mut archive = Archive::new(tar);
    archive.unpack(output_dir)?;
    fs::remove_file(&input_file)?;
    Ok(())
}
