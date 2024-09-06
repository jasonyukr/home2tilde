use std::env;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    // Get the user's home directory
    let home_dir = env::var("HOME").unwrap();

    // Regex to match escape sequences
    let escape_seq_re = Regex::new(r"\x1B\[[0-9;]*m$").unwrap();

    // Read from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(mut line) => {
                // Find the position of the home directory in the line
                if let Some(pos) = line.find(&home_dir) {
                    // Check if the prefix part is all escape sequence characters or if the position is 0
                    let prefix = &line[..pos];
                    if pos == 0 || escape_seq_re.is_match(prefix) {
                        // Replace home directory with "~"
                        line = line.replacen(&home_dir, "~", 1);
                    }
                }
                // Print the line, preserving escape sequences
                println!("{}", line);
            }
            Err(_e) => {}
        }
    }
}
