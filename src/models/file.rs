pub struct File 
{
    pub file_id: String,
    pub file_name: String,
    pub encrypted_files: Option<Vec<String>>, // Vector with direction of encrypted files
    pub enc_key: String,
}

impl File 
{
    pub fn new(file_id: String, file_name: String, encrypted_files: Option<Vec<String>>, enc_key: String) -> Self {
        File {
            file_id,
            file_name,
            encrypted_files,
            enc_key,
        }
    }   
}