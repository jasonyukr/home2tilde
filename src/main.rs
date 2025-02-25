use std::env;
use std::io::{self, BufRead};

#[derive(Debug)]
enum State {
    Normal,
    Escape,
    Csi,
}

fn convert_home_path(input_str: &str, homedir: &str) -> Option<String> {
    let input = input_str.as_bytes();
    let mut home_dir = homedir;

    let mut result = Vec::new();
    let mut state = State::Normal;
    let mut init_done = false;

    for &b in input {
        match state {
            State::Normal => {
                if b == 0x1B { // ESC (start of an escape sequence)
                    state = State::Escape;
                    result.push(b); // append ESC byte
                } else {
                    //////////////////////////////////////////////
                    if home_dir.len() > 0 {
                        let first_char_u8 = home_dir.chars().next().unwrap() as u8;
                        if first_char_u8 != b {
                            return None // mismatch detected
                        }
                        home_dir = &home_dir[1..];
                        if init_done == false {
                            result.push('~' as u8);
                            init_done = true;
                        }
                    } else {
                        result.push(b); // write normal byte to result
                    }
                    //////////////////////////////////////////////
                }
            }
            State::Escape => {
                if b == 0x5B { // '[' (CSI start)
                    state = State::Csi;
                    result.push(b); // append '['
                } else {
                    state = State::Normal;
                    result.push(b); // normal byte
                }
            }
            State::Csi => {
                if b >= 0x40 && b < 0x80 { // CSI control byte range (end of escape sequence)
                    state = State::Normal;
                    result.push(b); // append CSI control byte
                } else {
                    result.push(b); // append non-control CSI bytes
                }
            }
        }
    }

    match String::from_utf8(result) {
        Ok(str) => Some(str),
        Err(_e) => None,
    }
}

fn main() {
    // Get the user's home directory
    let home_dir = env::var("HOME").unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(ln) = line {
            if let Some(converted) = convert_home_path(&ln, &home_dir) {
                println!("{}", converted);
            } else {
                println!("{}", ln);
            }
        }
    }
}
