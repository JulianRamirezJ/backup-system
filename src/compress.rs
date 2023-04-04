use std::fs;
use std::io;
use std::fs::File;
use std::error::Error;
use tar::{Builder,Header, Archive};
use std::path::{Path, PathBuf};
use std::result::Result;
use zip::write::FileOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use zip::{ZipArchive, ZipWriter,result::ZipError};

fn compress(input_folder: &String, output_folder: &String) -> Result<String, std::io::Error>
{
    let last_folder_name = Path::new(input_folder)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

    let output_file = format!("{}/{}/{}.{}", output_folder, last_folder_name, last_folder_name, "zip");
    let compressed_folder = format!("{}/{}", output_folder, last_folder_name);
    let output_dir = Path::new(&compressed_folder);

    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }else{
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Folder Already Backuped"));
    }
    
    let file = File::create(output_file)?;
    let mut zip = ZipWriter::new(BufWriter::new(file));

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for entry in fs::read_dir(input_folder)? {
        let path = entry?.path();
        let mut file = File::open(&path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let name = path.file_name().unwrap().to_str().unwrap();
        zip.start_file(name, options)?;
        zip.write_all(&contents)?;use std::error::Error;
    }
    zip.finish()?;
    Ok(compressed_folder)
}

fn decompress(input_folder: &String, output_folder: &String) -> std::io::Result<()>
{
    let last_folder_name = Path::new(input_folder)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

    let input_file = format!("{}/{}.{}", input_folder, last_folder_name, "zip");

    let file = File::open(input_file)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let output_path = format!("{}/{}", output_folder, file.name());
        let mut output_file = File::create(&output_path)?;
        std::io::copy(&mut file, &mut output_file)?;
    }

    Ok(())
}