use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<i32> {
  let file = File::open("inputs/day1").unwrap();
  let lines = BufReader::new(file).lines();
  let parsed = lines
    .map(|result| result.unwrap().parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  parsed
}

pub fn puzzle1() {
  let numbers = parse_input();

  let sum: i32 = numbers.iter().sum();
  println!("Puzzle 1: {:?}", sum);
}

fn find_dupe(numbers: &Vec<i32>) -> i32 {
  let mut seen: HashSet<i32> = HashSet::new();
  let mut curr = 0;
  for x in numbers.iter().cycle() {
    curr += x;
    if seen.contains(&curr) {
      break;
    } else {
      seen.insert(curr);
    }
  }
  curr
}

pub fn puzzle2() {
  let numbers = parse_input();

  let dupe = find_dupe(&numbers);
  println!("Puzzle 2: {:?}", dupe);
}
