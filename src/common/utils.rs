use std::fs::File;
use std::fs;
use std::io::{BufRead, BufReader, Lines};

pub fn read_whole_file(file_name: &str) -> String{
    fs::read_to_string(file_name)
        .expect("Something went wrong reading the file")
}

pub fn read_lines_from_file(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}