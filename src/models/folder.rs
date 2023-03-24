
pub struct Folder 
{
    pub folder_path: String,
    pub file_names: Option<Vec<String>>,
}

impl Folder 
{
    pub fn new(folder_path: String, file_names:Option<Vec<String>>) -> Self {
        Folder {
            folder_path,
            file_names,
        }
    }

    pub fn add_file_name(&mut self, name: String) {
        if let Some(file_names) = &mut self.file_names {
            file_names.push(name);
        } else {
            self.file_names = Some(vec![name]);
        }
    }

    pub fn print_file_names(&self) {
        if let Some(names) = &self.file_names {
            for name in names {
                println!("File name: {}", name);
            }
        } else {
            println!("This folder has no files.");
        }
    }

}
