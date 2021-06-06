use regex::Regex;
use std::cmp::min;
use std::fs::File;
use std::io::Read;

const MAP_SIZE: usize = 30_000;
const SHIFT: isize = (MAP_SIZE / 2) as isize;

pub fn part1() -> usize {
  let data = parse();

  let path_a = path(&data[0]);
  let path_b = path(&data[1]);

  for n in 1..MAP_SIZE as isize / 2 {
    for i in 0..=n {
      if check(&path_a, &path_b, n, i)
        || check(&path_a, &path_b, n, -i)
        || check(&path_a, &path_b, -n, i)
        || check(&path_a, &path_b, -n, -i)
        || check(&path_a, &path_b, i, n)
        || check(&path_a, &path_b, -i, n)
        || check(&path_a, &path_b, i, -n)
        || check(&path_a, &path_b, -i, -n)
      {
        return (n + i) as usize;
      }
    }
  }

  panic!("No solution found for part 1")
}

pub fn part2() -> usize {
  let data = parse();

  let path_a = path(&data[0]);
  let path_b = path(&data[1]);

  let mut best = None;

  for i in 0..MAP_SIZE {
    for j in 0..MAP_SIZE {
      if path_a[i][j] > 0 && path_b[i][j] > 0 {
        match best {
          Some(b) => best = Some(min(b, path_a[i][j] + path_b[i][j])),
          None => best = Some(path_a[i][j] + path_b[i][j]),
        }
      }
    }
  }

  best.expect("No solution found for part 2")
}

fn check(path_a: &Vec<Vec<usize>>, path_b: &Vec<Vec<usize>>, x: isize, y: isize) -> bool {
  path_a[(SHIFT + x) as usize][(SHIFT + y) as usize] > 0
    && path_b[(SHIFT + x) as usize][(SHIFT + y) as usize] > 0
}

fn path(steps: &Vec<Step>) -> Vec<Vec<usize>> {
  let mut ret = vec![vec![0; MAP_SIZE]; MAP_SIZE];

  let mut curr = (MAP_SIZE / 2, MAP_SIZE / 2);

  let mut dist = 0;

  for step in steps {
    for _ in 0..step.len() {
      match step {
        Step::Down(_) => curr = (curr.0, curr.1 - 1),
        Step::Up(_) => curr = (curr.0, curr.1 + 1),
        Step::Left(_) => curr = (curr.0 - 1, curr.1),
        Step::Right(_) => curr = (curr.0 + 1, curr.1),
      }
      dist += 1;
      ret[curr.0][curr.1] = dist;
    }
  }

  ret
}

fn parse() -> Vec<Vec<Step>> {
  let mut content = String::new();

  File::open("res/day3.txt")
    .unwrap()
    .read_to_string(&mut content)
    .unwrap();

  content
    .split_ascii_whitespace()
    .map(|line| line.split(",").map(|s| Step::from(s)).collect())
    .collect()
}

#[derive(Debug)]
enum Step {
  Left(usize),
  Right(usize),
  Up(usize),
  Down(usize),
}

impl Step {
  pub fn len(&self) -> usize {
    match self {
      Self::Left(v) | Self::Right(v) | Self::Up(v) | Self::Down(v) => *v,
    }
  }
}

impl From<&str> for Step {
  fn from(s: &str) -> Self {
    let re = Regex::new("(L|R|U|D)([0-9]+)").unwrap();

    match re.captures(s) {
      Some(c) => {
        let n = c.get(2).unwrap().as_str().parse().unwrap();

        match c.get(1).unwrap().as_str() {
          "L" => Self::Left(n),
          "R" => Self::Right(n),
          "U" => Self::Up(n),
          "D" => Self::Down(n),
          _ => panic!("Couldn't parse '{}' as a step", s),
        }
      }
      None => panic!("Couldn't parse {} as a step", s),
    }
  }
}
