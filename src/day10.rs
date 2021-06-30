use std::{
  cmp::{max, min},
  collections::HashSet,
  fs::File,
  io::Read,
};

pub fn part1() -> usize {
  let asteroids = parse();

  let mut best = 0;

  for from in asteroids.iter() {
    let mut cnt = 0;

    for to in asteroids.iter() {
      if from == to {
        continue;
      }

      if is_visible(from, to, &asteroids) {
        cnt += 1;
      }
    }

    best = max(best, cnt);
  }

  best
}

pub fn part2() -> usize {
  42
}

fn is_visible(
  &(i, j): &(usize, usize),
  &(k, l): &(usize, usize),
  asteroids: &HashSet<(usize, usize)>,
) -> bool {
  let d_i = k as isize - i as isize;
  let d_j = l as isize - j as isize;

  let gcd = gcd(max(d_i.abs(), d_j.abs()), min(d_i.abs(), d_j.abs()));

  let step_i = d_i / gcd;
  let step_j = d_j / gcd;

  let mut point_i = (i as isize + step_i) as usize;
  let mut point_j = (j as isize + step_j) as usize;

  while point_i != k || point_j != l {
    if asteroids.contains(&(point_i, point_j)) {
      return false;
    }

    point_i = (point_i as isize + step_i) as usize;
    point_j = (point_j as isize + step_j) as usize;
  }

  true
}

fn gcd(a: isize, b: isize) -> isize {
  if b == 0 {
    return a;
  }

  gcd(b, a % b)
}

fn parse() -> HashSet<(usize, usize)> {
  let mut content = String::new();

  File::open("res/day10.txt")
    .unwrap()
    .read_to_string(&mut content)
    .unwrap();

  let mut map: Vec<Vec<char>> = vec![];

  for line in content.split_whitespace() {
    map.push(line.chars().collect());
  }

  let mut asteroids = HashSet::new();

  for i in 0..map.len() {
    for j in 0..map[i].len() {
      if map[i][j] == '.' {
        continue;
      }

      assert_eq!(map[i][j], '#');

      asteroids.insert((i, j));
    }
  }

  asteroids
}
