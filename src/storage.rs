use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

const NOTES_FILE: &str = "notes.txt";
const PASSWORD_FILE: &str = ".securepad_pass";

// Check if password file exists
pub fn password_exists() -> bool {
    Path::new(PASSWORD_FILE).exists()
}

// Save password hash to file
pub fn save_password_hash(hash: &str) -> io::Result<()> {
    let mut file = File::create(PASSWORD_FILE)?;
    file.write_all(hash.as_bytes())?;
    Ok(())
}

// Load password hash from file
pub fn load_password_hash() -> io::Result<String> {
    let mut file = File::open(PASSWORD_FILE)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Load notes from file
pub fn load_notes() -> io::Result<String> {
    if !Path::new(NOTES_FILE).exists() {
        return Ok(String::new());
    }
    
    let mut file = File::open(NOTES_FILE)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Save notes to file
pub fn save_notes(content: &str) -> io::Result<()> {
    let mut file = File::create(NOTES_FILE)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
