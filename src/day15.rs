use crate::{comp::IntcodeComputer, dir::Direction};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
}

impl Tile {
    fn is_wall(&self) -> bool {
        matches!(self, Self::Wall)
    }
}

impl From<isize> for Tile {
    fn from(from: isize) -> Self {
        match from {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::Oxygen,
            _ => panic!("Can't derive tile from {}", from),
        }
    }
}

impl From<Tile> for char {
    fn from(from: Tile) -> Self {
        match from {
            Tile::Empty => ' ',
            Tile::Oxygen => 'O',
            Tile::Wall => '▓',
        }
    }
}

fn neighbours(pos: (isize, isize)) -> Vec<((isize, isize), Direction)> {
    [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .iter()
    .map(|dir| {
        let step = dir.forward();

        ((pos.0 + step.0, pos.1 + step.1), *dir)
    })
    .collect()
}

fn dir_to_num(direction: &Direction) -> isize {
    match direction {
        Direction::North => 1,
        Direction::South => 2,
        Direction::West => 3,
        Direction::East => 4,
    }
}

pub fn part1() -> usize {
    let mut computer = IntcodeComputer::from_file("res/day15.txt");

    let mut curr = (0, 0);

    let mut map: HashMap<(isize, isize), Tile> = HashMap::new();
    map.insert(curr, Tile::Empty);

    let mut history: Vec<Direction> = vec![];

    loop {
        if let Some((next, dir)) = neighbours(curr)
            .into_iter()
            .find(|(n, _)| !map.contains_key(n))
        {
            let output = computer.execute(vec![dir_to_num(&dir)]);

            assert_eq!(output.len(), 1);

            let tile = output[0].into();

            map.insert(next, tile);

            if !tile.is_wall() {
                history.push(dir);
                curr = next;
            }
        } else if let Some(prev) = history.pop() {
            let flipped = prev.flip();
            let step = flipped.forward();
            curr.0 += step.0;
            curr.1 += step.1;

            let output = computer.execute(vec![dir_to_num(&flipped)]);
            assert_eq!(output.len(), 1);
            assert_eq!(Tile::from(output[0]), *map.get(&curr).unwrap());
        } else {
            break;
        }
    }

    // Map Ready
    // TODO: Ger rid of printing and build A*

    let min_i = map.keys().map(|pos| pos.0).min().unwrap();
    let max_i = map.keys().map(|pos| pos.0).max().unwrap();
    let min_j = map.keys().map(|pos| pos.1).min().unwrap();
    let max_j = map.keys().map(|pos| pos.1).max().unwrap();

    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if i == 0 && j == 0 {
                print!("X");
            } else if let Some(tile) = map.get(&(i, j)).map(|t| char::from(*t)) {
                print!("{}", tile);
            } else {
                print!("▓");
                map.insert((i, j), Tile::Wall);
            }
        }
        println!();
    }

    42
}

pub fn part2() -> usize {
    42
}
