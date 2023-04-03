/**
    Main program - Backup System
    Julian Andres Ramirez Jimenez
    Basic Directory for tests : /home/julianramirezj/backup-system/backup-folder/
**/

extern crate walkdir;

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


mod models;


fn main()  -> Result<(), std::io::Error>
{
    if let Some(args) = receive_folder()
    {

        let mode = &args[1];
        let input_folder = &args[2];
        let output_folder = &args[3];
        let pass = &args[4];

        match mode.as_str() {
            "mb" => {
                match create_tarball(&input_folder, &output_folder) {
                    Ok(output_file_path) => {
                        println!("Tarball created successfully. Output file path: {}", output_file_path);
                        match split_file(&output_file_path, 30 * 1024 * 1024) {
                            Ok(_) => Ok(()),
                            Err(err) => {
                                println!("Split failed with error: {}", err);
                                Err(err)
                            }
                        }
                    },
                    Err(err) => {
                        println!("Compression failed with error: {}", err);
                        Err(err)
                    }
                }
            },
            "rb" => {
                match restore_from_tarball(&input_folder, &output_folder) {
                    Ok(_) => {
                        println!("{}","Restored sucessfully");
                        Ok(())
                    },
                    Err(err) => {
                        println!("Restore failed with error: {}", err);
                        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, ""))
                    }
                }
            },
            _ => {
                Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid mode specified"))
            }
        }
        
    } 
    else
    {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No arguments found"))
    }
}

fn receive_folder() -> Option<Vec<String>>
{ 
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        println!("Usage: program_name arg1 arg2 arg3 arg4");
        None
    }else{
        Some(args)
    }
}

fn create_tarball(input_folder: &String, output_folder: &String) -> io::Result<String> {
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


fn restore_from_tarball(input_folder: &String, output_folder: &String) -> Result<(), Box<dyn Error>> {
    let last_folder_name = Path::new(input_folder)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

    let input_file = format!("{}/{}.{}", input_folder, last_folder_name, "tar");
    let output_dir = format!("{}/{}", output_folder, last_folder_name);
    let output_path = Path::new(&output_dir);
    fs::create_dir_all(output_dir.clone())?;
    let tar = std::fs::File::open(input_file)?;
    let mut archive = tar::Archive::new(tar);
    archive.unpack(output_dir)?;
    Ok(())
}


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

fn split_file(output_folder: &String, chunk_size: usize) -> Result<(), std::io::Error> {
    let mut input_file = File::open(&output_folder.clone())?;
    let mut buffer = [0; 1024];
    let mut chunk_number = 1;

    loop {
        let output_file_path = format!("{}_part{}", output_folder, chunk_number);
        let mut output_file = File::create(&output_file_path)?;

        let mut bytes_written = 0;
        while bytes_written < chunk_size {
            let bytes_to_read = std::cmp::min(buffer.len(), chunk_size - bytes_written);
            let bytes = input_file.read(&mut buffer[..bytes_to_read])?;
            if bytes == 0 {
                if bytes_written == 0 {
                    fs::remove_file(std::path::Path::new(&output_file_path));
                    fs::remove_file(std::path::Path::new(&output_folder.clone()));
                }
                break;
            }
            output_file.write_all(&buffer[..bytes])?;
            bytes_written += bytes;
        }
        if bytes_written == 0 {
            break;
        }
        chunk_number += 1;
    }

    Ok(())
}
