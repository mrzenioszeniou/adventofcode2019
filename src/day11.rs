use crate::{comp::IntcodeComputer, dir::Direction};
use std::collections::HashMap;

pub fn part1() -> usize {
  let mut robot = IntcodeComputer::from_file("res/day11.txt");

  let mut painted: HashMap<Position, Color> = HashMap::new();

  let mut state = State::default();

  while !robot.is_done() {
    let output = robot.execute(vec![painted
      .get(&state.position())
      .cloned()
      .unwrap_or(Color::Black)
      .into()]);

    assert_eq!(output.len(), 2);

    painted.insert(state.position(), output[0].into());

    match output[1] {
      0 => state.turn_left(),
      1 => state.turn_right(),
      _ => panic!("{} is not a valid turn", output[1]),
    }

    state.move_forward();
  }

  painted.len()
}

pub fn part2() -> String {
  let mut robot = IntcodeComputer::from_file("res/day11.txt");

  let mut painted: HashMap<Position, Color> = HashMap::new();

  let mut state = State::default();

  let mut first = true;

  while !robot.is_done() {
    let input = if first {
      first = false;
      vec![Color::White.into()]
    } else {
      vec![painted
        .get(&state.position())
        .cloned()
        .unwrap_or(Color::Black)
        .into()]
    };

    let output = robot.execute(input);

    assert_eq!(output.len(), 2);

    painted.insert(state.position(), output[0].into());

    match output[1] {
      0 => state.turn_left(),
      1 => state.turn_right(),
      _ => panic!("{} is not a valid turn", output[1]),
    }

    state.move_forward();
  }

  let mut min_i = 0;
  let mut max_i = 0;
  let mut min_j = 0;
  let mut max_j = 0;

  for tile in painted.keys() {
    min_i = std::cmp::min(tile.0, min_i);
    max_i = std::cmp::max(tile.0, max_i);
    min_j = std::cmp::min(tile.1, min_j);
    max_j = std::cmp::max(tile.1, max_j);
  }

  let mut ret = String::from("\n");

  for i in min_i..=max_i {
    for j in min_j..=max_j {
      ret.push(
        painted
          .get(&(i, j))
          .cloned()
          .unwrap_or(Color::Black)
          .to_char(),
      );
    }
    ret.push('\n');
  }

  ret
}

struct State {
  i: isize,
  j: isize,
  dir: Direction,
}

impl State {
  pub fn position(&self) -> Position {
    (self.i, self.j)
  }

  pub fn turn_right(&mut self) {
    self.dir = self.dir.right();
  }

  pub fn turn_left(&mut self) {
    self.dir = self.dir.left();
  }

  pub fn move_forward(&mut self) {
    let (di, dj) = self.dir.forward();

    self.i += di;
    self.j += dj;
  }
}

impl Default for State {
  fn default() -> Self {
    Self {
      i: 0,
      j: 0,
      dir: Direction::North,
    }
  }
}

#[derive(Clone, Copy)]
enum Color {
  Black,
  White,
}

impl Color {
  pub fn to_char(self) -> char {
    match self {
      Self::Black => '█',
      Self::White => ' ',
    }
  }
}

impl From<isize> for Color {
  fn from(from: isize) -> Self {
    match from {
      0 => Self::Black,
      1 => Self::White,
      _ => panic!("{} cannot be converted to a color", from),
    }
  }
}

impl From<Color> for isize {
  fn from(from: Color) -> Self {
    match from {
      Color::Black => 0,
      Color::White => 1,
    }
  }
}

type Position = (isize, isize);
