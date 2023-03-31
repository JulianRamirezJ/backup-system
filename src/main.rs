/**
    Main program - Backup System
    Julian Andres Ramirez Jimenez
    Basic Directory for tests : /home/julianramirezj/backup-system/backup-folder/
**/

extern crate walkdir;

use std::io;
use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::{Path, PathBuf};
use std::result::Result;
use std::io::prelude::*;
use std::io::BufWriter;
use zip::write::FileOptions;
use zip::write::ZipWriter;

mod models;


fn main()  -> std::io::Result<()>
{
    if let Some(args) = receive_folder(){
        let input_folder = &args[1];
        let output_folder = &args[2];
        let pass = &args[3];

        println!("Arguments: {}, {}, {}", input_folder , output_folder, pass);

        let last_folder_name = Path::new(output_folder)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");

        let compressed_file_path = format!("{}/{}/{}.{}", output_folder, last_folder_name, last_folder_name, "zip");
        let compressed_folder = format!("{}/{}", output_folder, last_folder_name);
        let output_dir = Path::new(&compressed_folder);
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        println!("{}",compressed_file_path);

        if let Ok(()) = compress(&input_folder, &compressed_file_path) {
            println!("{}","Compression sucessfully");
            Ok(())
        }else{
            println!("{}","Error Compressing");
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Great problem"))
        }
    } else{
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No arguments found"))
    }
}

fn receive_folder() -> Option<Vec<String>>
{ 
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        println!("Usage: program_name arg1 arg2 arg3");
        return None;
    }else{
        return Some(args);
    }
}

fn compress(input_folder: &String, output_file: &String) -> std::io::Result<()>
{
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
        zip.write_all(&contents)?;
    }
    zip.finish()?;
    Ok(())
}
