use std::{collections::HashSet, convert::TryFrom, fmt::Display};

use crate::{
    comp::IntcodeComputer,
    dir::{neighbours, Direction},
};

pub fn part1() -> isize {
    parse().2.iter().map(|(i, j)| i * j).sum()
}

pub fn part2() -> usize {
    let (start, scaffolds, intersections) = parse();

    let paths = find_path(start, scaffolds, &intersections);

    println!("{} paths found", paths.len());
    for (i, path) in paths.into_iter().enumerate().take(2) {
        print!("Path {}: ", i);
        for mov in trim_path(path) {
            print!("{} ", mov);
        }
        println!()
    }

    // let mut min_i = isize::MAX;
    // let mut max_i = isize::MIN;
    // let mut min_j = isize::MAX;
    // let mut max_j = isize::MIN;

    // for pos in start.unvisited.iter() {
    //     min_i = std::cmp::min(min_i, pos.0);
    //     max_i = std::cmp::max(max_i, pos.0);
    //     min_j = std::cmp::min(min_j, pos.1);
    //     max_j = std::cmp::max(max_j, pos.1);
    // }

    // for state in paths[0].iter().rev() {
    //     for i in min_i..=max_i {
    //         for j in min_j..=max_j {
    //             if state.pos == (i, j) {
    //                 print!("{}", char::from(state.dir));
    //             } else if state.unvisited.contains(&(i, j)) {
    //                 print!("#");
    //             } else {
    //                 print!(" ");
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    //     println!();
    //     std::thread::sleep(Duration::from_millis(250));
    // }

    42
}

fn find_path(
    state: State,
    unvisited: HashSet<Point>,
    intersections: &HashSet<Point>,
) -> Vec<Vec<Move>> {
    if unvisited.is_empty() {
        return vec![vec![]];
    }

    let mut ret = vec![];

    for (dir, rotation) in [
        (state.dir, None),
        (state.dir.left(), Some(Move::Left)),
        (state.dir.right(), Some(Move::Right)),
    ] {
        let mut unvisited = unvisited.clone();
        let mut curr = state.clone();
        let mut steps = 0;

        loop {
            let step = dir.forward();
            let next = (curr.pos.0 + step.0, curr.pos.1 + step.1);

            if unvisited.contains(&next) || intersections.contains(&next) {
                unvisited.remove(&next);
                curr = State { pos: next, dir };
                steps += 1;

                if intersections.contains(&next) {
                    break;
                }
            } else {
                break;
            }
        }

        if steps > 0 {
            find_path(curr, unvisited, intersections)
                .into_iter()
                .for_each(|mut path| {
                    path.push(Move::Forward(steps));
                    if let Some(rotation) = &rotation {
                        path.push(rotation.clone());
                    }
                    ret.push(path);
                });
        }
    }

    ret
}

fn trim_path(path: Vec<Move>) -> Vec<Move> {
    let mut ret = vec![];

    for mov in path.into_iter().rev() {
        match (&mov, ret.last_mut()) {
            (Move::Forward(steps), Some(Move::Forward(ref mut prev))) => {
                *prev += steps;
            }
            _ => {
                ret.push(mov);
            }
        }
    }

    ret
}

type Point = (isize, isize);

/// (Start, Scaffolds, Intersections)
fn parse() -> (State, HashSet<Point>, HashSet<Point>) {
    let mut computer = IntcodeComputer::from_file("res/day17.txt");

    let mut scaffolds = HashSet::new();
    let mut start = None;

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
            '^' | '<' | '>' | 'v' => {
                start = Some(State {
                    pos: (i, j),
                    dir: Direction::try_from(t).unwrap(),
                });
            }
            '#' => {
                scaffolds.insert((i, j));
            }
            _ => panic!("`{}` is not a valid tile character", t),
        }

        j += 1;
    }

    let mut intersections = HashSet::new();
    for scaffold in scaffolds.iter() {
        if neighbours(*scaffold)
            .iter()
            .all(|(n, _)| scaffolds.contains(n))
        {
            intersections.insert(*scaffold);
        }
    }

    (start.unwrap(), scaffolds, intersections)
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    pos: Point,
    dir: Direction,
}

#[derive(Clone, Debug)]
enum Move {
    Left,
    Right,
    Forward(usize),
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forward(n) => write!(f, "{}", n),
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        part2();
    }
}
