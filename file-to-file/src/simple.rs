use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn main() {

    println!("map start");

    let mut output_file1 = File::create("output1.txt").unwrap();
    BufReader::new(File::open("input.txt").unwrap()).lines().map(|line| {
        let line = line.unwrap();
        let tokens: Vec<_> = line.split(",").collect();
        tokens[0].to_string() + "," + tokens[2] + "," + tokens[1]
    }).for_each(|line| {
        writeln!(output_file1, "{}", line).unwrap();
    });

    println!("map end");

    println!("loop start");

    let mut output_file2 = File::create("output2.txt").unwrap();
    for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let tokens: Vec<_> = line.split(",").collect();
        writeln!(output_file2, "{}", tokens[0].to_string() + "," + tokens[2] + "," + tokens[1]).unwrap();
    }

    println!("loop end");
}
