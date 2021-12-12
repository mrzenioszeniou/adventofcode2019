use crate::{comp::IntcodeComputer, dir::Direction};
use std::collections::{BTreeMap, HashMap};

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

    fn is_oxygen(&self) -> bool {
        matches!(self, Self::Oxygen)
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

    let target = *map.iter().find(|(pos, tile)| tile.is_oxygen()).unwrap().0;
    let mut visited: HashMap<(isize, isize), (usize, (isize, isize))> = HashMap::new();
    let mut to_visit: BTreeMap<usize, (isize, isize)> =
        BTreeMap::from([((target.0.abs() + target.1.abs()) as usize, (0, 0))]);

    while let Some((curr_distance, mut curr)) = to_visit.pop_first() {
        if map.get(&curr).unwrap().is_oxygen() {
            let mut path = vec![curr];
            while curr != (0, 0) {
                curr = visited.get(&curr).unwrap().1;
                path.push(curr);
            }
            // println!("{:?}", path);
            return path.len() - 1;
        }

        for (neighbour, _) in neighbours(curr) {
            if map.get(&neighbour).unwrap_or(&Tile::Wall).is_wall() {
                continue;
            }

            if visited.get(&neighbour).map(|n| n.0).unwrap_or(usize::MAX) > curr_distance + 1 {
                visited.insert(neighbour, (curr_distance + 1, curr));
                to_visit.insert(
                    curr_distance
                        + 1
                        + (target.0 - neighbour.0).abs() as usize
                        + (target.1 - neighbour.1).abs() as usize,
                    neighbour,
                );
            }
        }
    }

    unreachable!();
}

pub fn part2() -> usize {
    42
}
