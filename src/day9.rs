use crate::comp::IntcodeComputer;

pub fn part1() -> isize {
  let mut computer = IntcodeComputer::from_file("res/day9.txt");

  let output = computer.execute(vec![1]);

  assert_eq!(
    output.len(),
    1,
    "Expected single output but found {:?}",
    output
  );

  output[0]
}

pub fn part2() -> isize {
  let mut computer = IntcodeComputer::from_file("res/day9.txt");

  let output = computer.execute(vec![2]);

  assert_eq!(
    output.len(),
    1,
    "Expected single output but found {:?}",
    output
  );

  output[0]
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn regression() {
    assert_eq!(part1(), 3380552333);
    assert_eq!(part2(), 78831);
  }
}
