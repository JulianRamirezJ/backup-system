//With classes

/**
    Main program - Backup System
    Julian Andres Ramirez Jimenez
    Basic Directory for tests : /home/julianramirezj/backup-system/backup-folder
**/

extern crate walkdir;
use walkdir::WalkDir;

use std::io;
use std::fs;

mod models;
use models::{File, Folder};


fn main() 
{
    let folder : String = receive_folder();
    println!("Folder: {}",folder);
    let mut folder_obj = Folder::new(folder.clone(), None);
    for file in WalkDir::new(folder).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            let file_name  = file.path().file_name().unwrap().to_str().unwrap();
            let metadata = fs::metadata(file.path()).unwrap();
            /*println!("Name : {} -- Size: {}", 
                    file_name,
                    metadata.len());*/
            folder_obj.add_file_name(file_name.to_string().clone());
            let mut file_obj = File::new(file_name.to_string().clone(),
                                         file_name.to_string(),
                                        );
        }
    }
    folder_obj.print_file_names();    
}

fn receive_folder() -> String 
{ 
    let mut folder = String::new();
    io::stdin()
    .read_line(&mut folder)
    .expect("Error");
    folder = folder.trim().to_string();
    return folder;
}
