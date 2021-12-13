use crate::{comp::IntcodeComputer, dir::Direction};
use std::collections::{BTreeSet, HashMap};

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

fn build_map(mut computer: IntcodeComputer) -> HashMap<(isize, isize), Tile> {
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

    map
}

fn a_star(
    map: &HashMap<(isize, isize), Tile>,
    start: (isize, isize),
    end: (isize, isize),
) -> Option<Vec<(isize, isize)>> {
    let mut prevs: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    let mut dists: HashMap<(isize, isize), usize> = HashMap::from([(start, 0)]);
    let mut to_visit: BTreeSet<(usize, (isize, isize))> = BTreeSet::from([(0, start)]);

    while let Some((_, mut curr)) = to_visit.pop_first() {
        if curr == end {
            let mut path = vec![curr];
            while curr != start {
                curr = *prevs.get(&curr).unwrap();
                path.push(curr);
            }
            return Some(path);
        }

        for (neighbour, _) in neighbours(curr) {
            if map.get(&neighbour).unwrap_or(&Tile::Wall).is_wall() {
                continue;
            }

            let curr_distance = *dists.get(&curr).unwrap();

            if *dists.get(&neighbour).unwrap_or(&usize::MAX) > curr_distance + 1 {
                dists.insert(neighbour, curr_distance + 1);
                prevs.insert(neighbour, curr);
                to_visit.insert((
                    curr_distance
                        + 1
                        + (end.0 - neighbour.0).abs() as usize
                        + (end.1 - neighbour.1).abs() as usize,
                    neighbour,
                ));
            }
        }
    }

    None
}

pub fn part1() -> usize {
    let map = build_map(IntcodeComputer::from_file("res/day15.txt"));

    let oxygen = *map.iter().find(|(_, tile)| tile.is_oxygen()).unwrap().0;

    let path = a_star(&map, (0, 0), oxygen).unwrap();

    path.len() - 1
}

pub fn part2() -> usize {
    let map = build_map(IntcodeComputer::from_file("res/day15.txt"));

    let oxygen = *map.iter().find(|(_, tile)| tile.is_oxygen()).unwrap().0;

    let mut max = vec![];

    for (pos, tile) in map.iter() {
        if tile.is_wall() {
            continue;
        }

        if let Some(path) = a_star(&map, *pos, oxygen) {
            if path.len() > max.len() {
                max = path;
            }
        }
    }

    max.len() - 1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let map = HashMap::from([
            ((0, 1), Tile::Wall),
            ((0, 2), Tile::Wall),
            ((1, 0), Tile::Wall),
            ((1, 1), Tile::Empty),
            ((1, 2), Tile::Empty),
            ((1, 3), Tile::Wall),
            ((1, 4), Tile::Wall),
            ((2, 0), Tile::Wall),
            ((2, 1), Tile::Empty),
            ((2, 2), Tile::Wall),
            ((2, 3), Tile::Empty),
            ((2, 4), Tile::Empty),
            ((2, 5), Tile::Wall),
            ((3, 0), Tile::Wall),
            ((3, 1), Tile::Empty),
            ((3, 2), Tile::Oxygen),
            ((3, 3), Tile::Empty),
            ((3, 4), Tile::Wall),
            ((4, 1), Tile::Wall),
            ((4, 2), Tile::Wall),
            ((4, 3), Tile::Wall),
        ]);

        let oxygen = *map.iter().find(|(_, tile)| tile.is_oxygen()).unwrap().0;

        let mut max = 0;

        for (pos, tile) in map.iter() {
            if tile.is_wall() {
                continue;
            }

            if let Some(path) = a_star(&map, oxygen, *pos) {
                max = std::cmp::max(path.len() - 1, max);
            }
        }

        assert_eq!(max, 4);
    }

    #[test]
    fn part_two() {
        part2();
    }
}
