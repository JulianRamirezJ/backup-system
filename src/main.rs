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
use serde::{Deserialize, Serialize};
use serde_json::json;


mod models;


#[derive(Deserialize, Serialize)]
struct BackupInfo {
    input_folder: String,
    output_file: String,
    chunk_size: usize,
    split_files: Vec<String>,
    password: String,
}

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
                        match split_file(&output_file_path, 30 * 1024 * 1024,pass.clone()) {
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
                match reassemble_tar_file(input_folder){
                    Ok(_) => {
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
                    Err(err) => Err(err)
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

fn split_file(output_folder: &String, chunk_size: usize, pass: String) -> Result<(), std::io::Error> {
    let mut input_file = File::open(&output_folder.clone())?;
    let input_file_path = Path::new(output_folder);
    let mut buffer = [0; 1024];
    let mut chunk_number = 1;
    let mut split_files = Vec::new();

    loop {
        let output_file_path = format!("{}_part{}", output_folder, chunk_number);
        let output_file_path_ = Path::new(&output_file_path);
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
        let file_name = output_file_path_.file_name().unwrap().to_str().unwrap().to_owned();
        split_files.push(file_name);
    }

    let path = input_file_path.parent().unwrap().to_str().unwrap();
    let json_data = json!({
        "input_folder": path,
        "output_file": output_folder,
        "chunk_size": chunk_size,
        "split_files": split_files,
        "password":pass
    });
    let parent = input_file_path.parent().unwrap().file_name().unwrap().to_str().unwrap();
    println!("{}",parent);
    let json_file_path = format!("{}/{}.json","/home/julianramirezj/backup-system/info",parent);
    let mut json_file = File::create(json_file_path)?;
    serde_json::to_writer_pretty(&mut json_file, &json_data)?;

    Ok(())
}

fn reassemble_tar_file(path: &String) -> io::Result<()> {

    let input_path = Path::new(path);
    let file_name = input_path.file_name().unwrap().to_str().unwrap();
    println!("{}",file_name);
    let info_file_path = format!("{}/{}.json","/home/julianramirezj/backup-system/info",file_name);
    println!("{}",info_file_path);
    let info_file = File::open(info_file_path)?;
    let info: BackupInfo = serde_json::from_reader(info_file)?;
    let output_file_path = Path::new(&info.output_file);
    let mut output_file = File::create(&output_file_path)?;
    for split_file in info.split_files {
        let split_file_path = format!("{}/{}",info.input_folder.clone(), split_file);
        println!("{}",split_file_path);
        let mut input_file = File::open(&split_file_path)?;
        let mut buffer = [0; 1024];
        loop {
            let bytes = input_file.read(&mut buffer)?;
            if bytes == 0 {
                break;
            }
            output_file.write_all(&buffer[..bytes])?;
        }
        fs::remove_file(&split_file_path)?;
    }
    Ok(())
}
