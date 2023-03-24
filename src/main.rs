extern crate walkdir;
use walkdir::WalkDir;

use std::io;
use std::fs;


fn main() 
{
    let folder : String = receiveFolder();
    for file in WalkDir::new(folder).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            println!("{}", file.path().display());
        }
    }
    println!(folder);
}

fn receiveFolder() -> String 
{ 
    //let folder = "/home/julianramirezj/backup-system/backup-folder"
    let mut folder = String::new();
    io::stdin()
    .read_line(&mut folder)
    .expect("Error");
    folder = folder.trim().to_string()
}
