/**
    Main program - Backup System
    Julian Andres Ramirez Jimenez
    Basic Directory backup save test : /home/julianramirezj/backup-system-api/backup/
    Basic Directory backup restore test : /home/julianramirezj/output-backup
**/

use std::result::Result;
use openssl::hash::{MessageDigest};
use openssl::pkcs5::pbkdf2_hmac;
use std::path::{Path};
use rocket::{post, routes};
use rocket::serde::json::{Json, Value};
use rocket::http::Status;
use serde_json::json;

mod utils;
use crate::utils::{create_tarball, restore_from_tarball, 
                    split_file, reassemble_file,
                    encrypt_folder, decrypt_folder};

#[derive(serde::Serialize, serde::Deserialize)]
struct BackupRequest {
    input_folder: String,
    output_folder: String,
    pass: String,
}

#[post("/backup/create", format = "json", data = "<request>")]
fn create_backup(request: Json<BackupRequest>) -> Result<Json<Value>, (Status, String)>{
    let input_folder = request.input_folder.clone();
    let output_folder = request.output_folder.clone();
    let pass = request.pass.clone().to_string();
    let key = generate_key(pass.as_str());
    let info_folder = get_info_folder();

    match create_tarball(&input_folder, &output_folder) {
        Ok(output_file_path) => {
            println!("Tarball created successfully. Output file path: {}", output_file_path);
            match split_file(&output_file_path, 50 * 1000 * 1000, pass.clone(), key.clone(), info_folder) {
                Ok(folder) => {
                    println!("Files succesfully splitted");
                    encrypt_folder(folder.as_str(), &key).unwrap();
                    println!("Files succesfully encrypted");
                    Ok(Json(json!({
                        "status": "success",
                        "message": format!("Backup created for input_folder {},
                            output_folder {} with pass {}",
                            request.input_folder, request.output_folder, request.pass),
                    })))
                },
                Err(err) => {
                    println!("Split failed with error: {}", err);
                    Err((Status::InternalServerError, "Something failed".to_string()))
                }
            }
        },
        Err(err) => {
            println!("Compression failed with error: {}", err);
            Err((Status::InternalServerError, "Something failed".to_string()))
        }
    }
}

#[post("/backup/restore", format = "json", data = "<request>")]
fn restore_backup(request: Json<BackupRequest>) -> Result<Json<Value>, (Status, String)> {
    let input_folder = request.input_folder.clone();
    let output_folder = request.output_folder.clone();
    let pass = request.pass.clone().to_string();
    let info_folder = get_info_folder();

    decrypt_folder(input_folder.clone().as_str(), pass.clone(), info_folder.clone()).unwrap();
    println!("Files succesfully decrypted");
    match reassemble_file(&input_folder, info_folder.clone()){
        Ok(_) => {
            println!("Tarbal has been reassembled");
            match restore_from_tarball(&input_folder, &output_folder) {
                Ok(_) => {
                    println!("{}:{}","Restored sucessfully",output_folder);
                    Ok(Json(json!({
                        "status": "success",
                        "message": format!("Backup restored for input_folder {}, 
                        output_folder {} with pass {}", 
                        request.input_folder, request.output_folder, request.pass),
                    })))
                },
                Err(err) => {
                    println!("Restore failed with error: {}", err);
                    Err((Status::InternalServerError, "Something failed".to_string()))
                }
            }
        },
        Err(_err) => Err((Status::InternalServerError, "Something failed".to_string()))
    }
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![create_backup, restore_backup])
        .launch()
        .await;
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

