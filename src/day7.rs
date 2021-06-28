use std::cmp::max;
use std::{fs::File, io::Read};

use crate::comb::permutations;
use crate::comp::IntcodeComputer;

pub fn part1() -> usize {
  let base = parse();

  let mut best = 0;

  for (i, phase_settings) in permutations(&[0, 1, 2, 3, 4]).into_iter().enumerate() {
    println!("[{:04}]:{:?}", i, phase_settings);

    let out_0 = base.clone().execute(vec![0, phase_settings[0]]);
    assert_eq!(out_0.len(), 1);

    let out_1 = base.clone().execute(vec![out_0[0], phase_settings[1]]);
    assert_eq!(out_1.len(), 1);

    let out_2 = base.clone().execute(vec![out_1[0], phase_settings[2]]);
    assert_eq!(out_2.len(), 1);

    let out_3 = base.clone().execute(vec![out_2[0], phase_settings[3]]);
    assert_eq!(out_3.len(), 1);

    let out_4 = base.clone().execute(vec![out_3[0], phase_settings[4]]);
    assert_eq!(out_4.len(), 1);

    best = max(best, out_4[0]);
  }

  best as usize
}

pub fn part2() -> usize {
  42
}

fn parse() -> IntcodeComputer {
  let mut content = String::new();

  File::open("res/day7.txt")
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
