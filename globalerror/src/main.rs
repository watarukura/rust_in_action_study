#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display};

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// fileは、アクセス可能なファイルを意味する
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    /// 新規ファイルは空とみなすが、ファイル名は必須
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}({})>", self.name, self.state)
    }
}


fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut f6 = File::new("6.txt");

    let mut buffer: Vec<u8> = vec![];

    if f6.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f6 = open(f6).unwrap();
    let f6_length = f6.read(&mut buffer).unwrap();
    f6 = close(f6).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f6);
    println!("{}", f6);
    println!("{}", f6_length);
    println!("{}", text);
}
