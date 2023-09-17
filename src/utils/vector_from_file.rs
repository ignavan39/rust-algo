use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn vector_from_file(filename: impl AsRef<Path>) -> Vec<Vec<i32>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|t| {
            t.split(" ")
                .map(|t| t.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
}
