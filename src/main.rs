use std::env;
use std::io::{self, BufRead};

fn main() {
    // Get the user's home directory
    let home_dir = env::var("HOME").unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(mut ln) = line {
            let strip_ln = strip_ansi_escapes::strip_str(ln.clone());
            // Find the position of the home directory in the line
            if let Some(pos) = strip_ln.find(&home_dir) {
                if pos == 0 {
                    // Replace home directory with "~"
                    ln = ln.replacen(&home_dir, "~", 1);
                }
            }
            // Print the line, preserving escape sequences
            println!("{}", ln);
        }
    }
}
