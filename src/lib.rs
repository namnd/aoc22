use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn read_lines_from_file(day: &str, use_sample: bool) -> Result<Vec<String>> {
    let mut path = "inputs/".to_owned() + day;
    if use_sample {
        path += "_sample";
    }
    path += ".txt";
    let reader = BufReader::new(File::open(path).expect("Cannot open file"));
    let mut v = Vec::new();
    for line in reader.lines() {
        v.push(line.unwrap());
    }
    Ok(v)
}
