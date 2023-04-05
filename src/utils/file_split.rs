use std::fs;
use std::io;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use serde_json;

use super::backup_info::BackupInfo;

pub fn split_file(output_folder: &String, chunk_size: usize, pass:String, key:Vec<u8>) -> Result<String, std::io::Error> {
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
                    fs::remove_file(std::path::Path::new(&output_file_path)).unwrap();
                    fs::remove_file(std::path::Path::new(&output_folder.clone())).unwrap();

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
    let json_data = BackupInfo {
        input_folder: path.to_string(),
        output_file: output_folder.to_string(),
        chunk_size,
        split_files,
        pass,
        key
    };
    let parent = input_file_path.parent().unwrap().file_name().unwrap().to_str().unwrap();
    println!("{}",parent);
    let json_file_path = format!("{}/{}.json","/home/julianramirezj/backup-system/info",parent);
    let mut json_file = File::create(json_file_path)?;
    serde_json::to_writer_pretty(&mut json_file, &json_data)?;

    Ok(path.to_string())
}

pub fn reassemble_file(path: &String) -> io::Result<()> {

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
    }
    Ok(())
}