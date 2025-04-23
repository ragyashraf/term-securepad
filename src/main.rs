mod editor;
mod storage;

use editor::Editor;
use storage::{load_password_hash, save_password_hash, password_exists};
use std::io::{self, Write};
use rpassword::read_password;
use sha2::{Sha256, Digest};

fn main() {
    // Check if password exists, handle first-time setup if needed
    if !password_exists() {
        setup_password();
    } else if !verify_password() {
        println!("Invalid password. Exiting.");
        return;
    }

    // Password verified, start the editor
    match Editor::new() {
        Ok(mut editor) => {
            if let Err(e) = editor.run() {
                eprintln!("Error running editor: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize editor: {}", e);
        }
    }
}

fn setup_password() {
    println!("First time setup - Create a master password");
    print!("Enter new password: ");
    io::stdout().flush().unwrap();
    
    let password = read_password().unwrap();
    if password.trim().is_empty() {
        println!("Password cannot be empty. Please try again.");
        return setup_password();
    }
    
    print!("Confirm password: ");
    io::stdout().flush().unwrap();
    let confirm = read_password().unwrap();
    
    if password != confirm {
        println!("Passwords do not match. Please try again.");
        return setup_password();
    }
    
    // Hash the password before storing
    let mut hasher = Sha256::new();
    hasher.update(password);
    let hash = format!("{:x}", hasher.finalize());
    
    if let Err(e) = save_password_hash(&hash) {
        eprintln!("Failed to save password: {}", e);
        std::process::exit(1);
    }
    
    println!("Password created successfully!");
}

fn verify_password() -> bool {
    print!("Enter password to unlock your notes: ");
    io::stdout().flush().unwrap();
    
    let password = match read_password() {
        Ok(pass) => pass,
        Err(_) => return false,
    };
    
    // Hash the entered password for comparison
    let mut hasher = Sha256::new();
    hasher.update(password);
    let hash = format!("{:x}", hasher.finalize());
    
    match load_password_hash() {
        Ok(stored_hash) => hash == stored_hash,
        Err(e) => {
            eprintln!("Error loading password: {}", e);
            false
        }
    }
}
