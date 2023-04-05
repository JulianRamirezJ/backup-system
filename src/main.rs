/**
    Main program - Backup System
    Julian Andres Ramirez Jimenez
    Basic Directory for tests : /home/julianramirezj/backup-system/backup-folder/
**/

use std::result::Result;
use openssl::hash::{MessageDigest};
use openssl::pkcs5::pbkdf2_hmac;
use std::path::{Path};



mod utils;
use crate::utils::{create_tarball, restore_from_tarball, 
                    split_file, reassemble_file,
                    encrypt_folder, decrypt_folder};

fn main()  -> Result<(), std::io::Error>
{
    if let Some(args) = receive_folder()
    {

        let mode = &args[1];
        let input_folder = &args[2];
        let output_folder = &args[3];
        let pass = &args[4].to_string();
        let key = generate_key(pass.as_str());
        let info_folder = get_info_folder();

        match mode.as_str() {
            "mb" => {
                match create_tarball(&input_folder, &output_folder) {
                    Ok(output_file_path) => {
                        println!("Tarball created successfully. Output file path: {}", output_file_path);
                        match split_file(&output_file_path, 30 * 1024 * 1024, pass.clone(), key.clone(), info_folder) {
                            Ok(folder) => {
                                println!("Files succesfully splitted");
                                encrypt_folder(folder.as_str(), &key).unwrap();
                                println!("Files succesfully encrypted");
                                Ok(())
                            },
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
                decrypt_folder(input_folder.clone().as_str(), pass.clone(), info_folder.clone())?;
                println!("Files succesfully decrypted");
                match reassemble_file(input_folder, info_folder.clone()){
                    Ok(_) => {
                        println!("Tarbal has been reassembled");
                        match restore_from_tarball(&input_folder, &output_folder) {
                            Ok(_) => {
                                println!("{}:{}","Restored sucessfully",output_folder);
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

pub fn generate_key(password: &str) -> Vec<u8> {
    let mut key = vec![0; 16];
    let salt = b"this-is-my-salt";
    pbkdf2_hmac(password.as_bytes(), salt, 1000, MessageDigest::sha256(), &mut key).unwrap();
    key
}

pub fn get_info_folder() -> String {
    let current_dir = std::env::current_dir().unwrap();
    let info_folder = current_dir.join("info");
    if !info_folder.exists() {
        std::fs::create_dir(&info_folder).unwrap();
    }
    let absolute_path = Path::new(&info_folder).canonicalize().unwrap();
    absolute_path.to_str().unwrap().to_string()
}

