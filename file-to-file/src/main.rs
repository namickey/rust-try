mod simple;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::vec;

fn main() {

    simple::main();

    let input_file = File::open("input.txt").unwrap();
    let mut list = vec![];
    for line in BufReader::new(input_file).lines() {
        println!("{:?}", line);
        list.push(line);
    }

    let mut out_list: Vec<String> = vec![];
    for line in list {
        let line = line.unwrap();
        let tokens: Vec<_> = line.split(",").collect();
        out_list.push(tokens[0].to_string() + "," + tokens[2] + "," + tokens[1]);
    }

    let mut output_file = File::create("output.txt").unwrap();
    for line in out_list {
        writeln!(output_file, "{}", line).expect("Failed to write file");
    }
}

