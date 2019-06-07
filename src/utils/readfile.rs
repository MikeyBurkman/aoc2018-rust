use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_into_lines(fname: &str) -> Vec<String> {
  let file = File::open(fname).unwrap();
  let lines = BufReader::new(file).lines();
  let parsed = lines
    .map(|result| result.unwrap())
    .collect();
  parsed
}
