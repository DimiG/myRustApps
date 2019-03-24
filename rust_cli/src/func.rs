//!
//! This func.rs file for functions
//!

use std::process::Command;

pub fn list() {
    let mut cmd = Command::new("ls");

    cmd.arg("-la");

    match cmd.output() {
        // Output routine
        Ok(o) => unsafe {
            println!(
                "\nProgram output is ->\n\n {}",
                String::from_utf8_unchecked(o.stdout)
            );
        },

        Err(e) => {
            println!("There was an error! {}", e);
        }
    }
}
