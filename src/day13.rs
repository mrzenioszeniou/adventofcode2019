use std::collections::HashMap;

use crate::comp::IntcodeComputer;

pub fn part1() -> usize {
  let mut arcade = IntcodeComputer::from_file("res/day13.txt");

  let output = arcade.execute(vec![]);

  let mut tiles: HashMap<(isize, isize), isize> = HashMap::new();

  assert!(arcade.is_done());
  assert_eq!(output.len() % 3, 0);

  for i in (0..output.len()).step_by(3) {
    tiles.insert((output[i], output[i + 1]), output[i + 2]);
  }

  tiles.values().filter(|tile| **tile == 2).count()
}

pub fn part2() -> usize {
  42
}
