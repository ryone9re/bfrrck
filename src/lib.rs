use std::{
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
pub enum BfError {
    InvalidCharacter,
    IoError(io::Error),
    OtherError(&'static str),
}

impl From<io::Error> for BfError {
    fn from(error: io::Error) -> Self {
        BfError::IoError(error)
    }
}

pub fn exec(arg: &str) -> Result<(), BfError> {
    let mut memory: [u8; 30000] = [0; 30000];

    let mut buf = String::new();
    let mut f = File::open(arg)?;
    f.read_to_string(&mut buf)?;

    let mut index: usize = 0;

    let mut p = 0;
    let len = buf.len();

    while p < len {
        match buf.chars().nth(p) {
            Some('+') => {
                if memory[index] == 255 {
                    memory[index] = 0;
                } else {
                    memory[index] += 1;
                };
            }
            Some('-') => {
                if memory[index] == 0 {
                    memory[index] = 255;
                } else {
                    memory[index] -= 1;
                };
            }
            Some('>') => {
                index += 1;
            }
            Some('<') => {
                if index == 0 {
                    return Err(BfError::OtherError("Segmentation fault"));
                }
                index -= 1;
            }
            Some('[') => {
                if memory[index] == 0 {
                    match serch_into_right(&buf, p) {
                        Some(x) => {
                            p = x;
                        }
                        _ => return Err(BfError::OtherError("Syntax error")),
                    }
                }
            }
            Some(']') => {
                if memory[index] != 0 {
                    match serch_from_left(&buf, p) {
                        Some(x) => {
                            p = x;
                        }
                        _ => return Err(BfError::OtherError("Syntax error")),
                    }
                }
            }
            Some(',') => match get_char() {
                Ok(x) => memory[index] = x,
                Err(e) => return Err(e),
            },
            Some('.') => {
                print!("{}", memory[index] as char)
            }
            _ => {}
        }
        p += 1;
    }

    Ok(())
}

fn serch_from_left(s: &str, start_index: usize) -> Option<usize> {
    let mut right: usize = 0;

    for (i, c) in s[..start_index].chars().rev().enumerate() {
        if c == ']' {
            right += 1;
        }
        if c == '[' {
            if right == 0 {
                return Some(start_index - i - 1);
            }
            right -= 1;
        }
    }
    None
}

fn serch_into_right(s: &str, start_index: usize) -> Option<usize> {
    let mut left: usize = 0;

    for (i, c) in s[start_index + 1..].chars().enumerate() {
        if c == '[' {
            left += 1;
        }
        if c == ']' {
            if left == 0 {
                return Some(start_index + i + 1);
            }
            left -= 1;
        }
    }
    None
}

fn get_char() -> Result<u8, BfError> {
    let mut stdin = io::stdin();
    let mut buf: [u8; 1] = [0; 1];

    match stdin.read_exact(&mut buf) {
        Ok(_) => Ok(buf[0]),
        Err(e) => Err(BfError::IoError(e)),
    }
}
