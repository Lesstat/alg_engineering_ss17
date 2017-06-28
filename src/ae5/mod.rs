
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

use ndarray::Array2;

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

pub type PolyLine = Vec<Point>;


pub fn read_file<P: AsRef<Path>>(path: P) -> PolyLine {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("first line not present");
    buf.clear();
    let mut result = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let mut values = line.split(' ');
            let x = values.next().expect("no x value").parse().expect(
                "x value could not be parsed",
            );
            let y = values.next().expect("no y vaule").parse().expect(
                "y value could not be parsed",
            );
            result.push(Point { x, y });
        }
    }




    result
}
