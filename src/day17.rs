use std::collections::HashSet;

use crate::{comp::IntcodeComputer, dir::neighbours};

pub fn part1() -> isize {
    let mut computer = IntcodeComputer::from_file("res/day17.txt");

    let mut scaffolds: HashSet<(isize, isize)> = HashSet::new();

    let mut i = 0;
    let mut j = 0;

    for c in computer.execute(vec![]) {
        let t = char::from(c as u8);

        print!("{}", t);

        match t {
            '.' => {}
            '\n' => {
                i += 1;
                j = 0;
                continue;
            }
            '>' | '^' | '<' | 'v' | '#' => {
                scaffolds.insert((i, j));
            }
            _ => panic!("`{}` is not a valid tile character", t),
        }

        j += 1;
    }

    let mut sum = 0;

    for scaffold in scaffolds.iter() {
        if neighbours(*scaffold)
            .iter()
            .all(|(n, _)| scaffolds.contains(n))
        {
            sum += scaffold.0 * scaffold.1;
        }
    }

    sum
}

pub fn part2() -> usize {
    42
}
