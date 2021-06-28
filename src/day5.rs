use crate::comp::IntcodeComputer;
use std::{fs::File, io::Read};

pub fn part1() -> isize {
  let mut computer = parse();

  let output = computer.execute(vec![1]);

  let non_zero: Vec<isize> = output.into_iter().skip_while(|n| *n == 0).collect();

  assert_eq!(non_zero.len(), 1);

  non_zero[0].to_owned()
}

pub fn part2() -> isize {
  let mut computer = parse();

  let output = computer.execute(vec![5]);

  assert_eq!(output.len(), 1);

  output[0].to_owned()
}

fn parse() -> IntcodeComputer {
  let mut content = String::new();

  File::open("res/day5.txt")
    .unwrap()
    .read_to_string(&mut content)
    .unwrap();

  let memory = content
    .trim()
    .split(",")
    .map(|s| s.parse::<isize>().unwrap())
    .collect();

  IntcodeComputer::new(&memory)
}
