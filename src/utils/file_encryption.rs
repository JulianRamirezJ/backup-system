extern crate openssl;

use std::fs;
use openssl::symm::{Cipher, Crypter, Mode};
use std::fs::{File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use rayon::prelude::*;
use serde_json;

use super::backup_info::BackupInfo;


static IV: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

pub fn encrypt_folder(folder_path: &str, key: &[u8]) -> std::io::Result<()> {
    let files: Vec<_> = fs::read_dir(&folder_path)?.collect();
    files.par_iter()
        .try_for_each(|entry| -> std::io::Result<()> {
            let path = entry.as_ref().unwrap().path();
            encrypt_file(path.to_str().unwrap(), key)?;
            fs::remove_file(&path)?;
            Ok(())
        })
}

pub fn decrypt_folder(folder_path: &str, pass: String, info_folder: String) -> std::io::Result<()> {
    let input_path = Path::new(folder_path);
    let file_name = input_path.file_name().unwrap().to_str().unwrap();
    let info_file_path = format!("{}/{}.json",info_folder,file_name);
    let info_file = File::open(info_file_path)?;
    let info: BackupInfo = serde_json::from_reader(info_file)?;
    if pass == info.pass {
        let files: Vec<_> = fs::read_dir(&info.input_folder)?.collect();
            files.par_iter()
            .try_for_each(|entry| -> std::io::Result<()> {
                let path = entry.as_ref().unwrap().path();
                decrypt_file(path.to_str().unwrap(), info.input_folder.clone(), &info.key)?;
                Ok(())
            })
    }else{
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Something wrong"))
    }
}

fn encrypt_file(file_path: &str, key: &[u8]) -> std::io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    
    let cipher = Cipher::aes_128_cbc();
    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key, Some(&IV))?;
    let mut ciphertext = vec![0; contents.len() + cipher.block_size()];
    let count = encrypter.update(&contents, &mut ciphertext)?;
    let final_count = encrypter.finalize(&mut ciphertext[count..])?;
    ciphertext.truncate(count + final_count);
    
    let mut output_file = File::create(format!("{}.enc", file_path))?;
    output_file.write_all(&ciphertext)?;
    
    Ok(())
}

fn decrypt_file(file_path: &str, output_folder:String, key: &[u8]) -> std::io::Result<()> {
    let input_file_path = Path::new(file_path);
    let mut output_file_path = PathBuf::new();
    output_file_path.set_file_name(input_file_path.file_stem().unwrap());
    let output_file = format!("{}/{}",output_folder,output_file_path.display());
    let mut file = File::open(input_file_path)?;
    let mut contents = Vec::new();

    file.read_to_end(&mut contents)?;
    let cipher = Cipher::aes_128_cbc();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key, Some(&IV))?;
    let mut plaintext = vec![0; contents.len() + cipher.block_size()];

    let count = decrypter.update(&contents, &mut plaintext)?;
    let final_count = decrypter.finalize(&mut plaintext[count..])?;
    
    plaintext.truncate(count + final_count);
    let mut output_file = File::create(output_file)?;
    output_file.write_all(&plaintext)?;
    
    Ok(())
}

