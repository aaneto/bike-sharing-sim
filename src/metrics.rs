use std::{
    io::Write,
    fs::File,
};

pub fn write_to_file(metrics: Vec<crate::Metric>) {
    let text = serde_json::to_string(&metrics).unwrap();

    let err_msg = "Cannot create graph file";
    File::create("metrics.json")
        .expect(err_msg)
        .write_all(&text.as_bytes())
        .expect(err_msg);
}