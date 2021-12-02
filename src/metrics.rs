
use serde::Serialize;
use std::{
    io::Write,
    fs::File,
};

pub fn write_to_file<T: Serialize>(metrics: T, filename: &str) {
    let text = serde_json::to_string(&metrics).unwrap();
    File::create(filename)
        .expect("Cannot write file")
        .write_all(&text.as_bytes())
        .expect("Cannot write file");
}
