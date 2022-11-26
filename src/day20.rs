use std::collections::{HashMap, HashSet};

pub fn part1() -> usize {
    todo!()
}

pub fn part2() -> usize {
    todo!()
}

fn parse() -> Map {
    let mut portals: HashMap<char, Vec<Position>> = HashMap::new();
    let mut spaces = HashSet::new();

    for (i, row) in std::fs::read_to_string("res/day20.txt")
        .unwrap()
        .split('\n')
        .enumerate()
    {
        for (j, c) in row.chars().enumerate() {
            match c {
                ' ' | '#' => {}
                '.' => {
                    spaces.insert(Position(i, j));
                }
                'A'..='Z' => portals.entry(c).or_default().push(Position(i, j)),
                _ => panic!("Unexpected character `{}`", c),
            }
        }
    }

    todo!()
}

struct Map {
    portals: HashMap<Position, Position>,
    spaces: HashSet<Position>,
    start: Position,
    end: Position,
}

#[derive(PartialEq, Eq, Hash)]
struct Position(usize, usize);
