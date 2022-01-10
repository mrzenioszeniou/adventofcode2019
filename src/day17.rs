use std::{
    collections::HashSet,
    convert::TryFrom,
    fmt::{Debug, Display},
};

use crate::{
    comp::IntcodeComputer,
    dir::{neighbours, Direction},
};

pub fn part1() -> isize {
    let mut computer = IntcodeComputer::from_file("res/day17.txt");
    let output: String = computer
        .execute(vec![])
        .into_iter()
        .map(|b| char::from(b as u8))
        .collect();

    parse(&output).2.iter().map(|(i, j)| i * j).sum()
}

pub fn part2() -> isize {
    let mut computer = IntcodeComputer::from_file("res/day17.txt");
    let output: String = computer
        .execute(vec![])
        .into_iter()
        .map(|b| char::from(b as u8))
        .collect();

    let (start, scaffolds, intersections) = parse(&output);

    let mut paths = find_path(start, scaffolds, &intersections)
        .into_iter()
        .map(trim_path)
        .collect::<Vec<_>>();

    paths.sort_by_key(|p| p.len());

    for (_, path) in paths.into_iter().enumerate() {
        if let Some((functions, routine)) = get_funcs(&path, &[], &[]) {
            let mut input: String = String::new();
            for f in routine {
                input.push((b'A' + f as u8) as char);
                input.push(',');
            }
            input.pop();
            input.push('\n');

            for function in functions {
                for m in function {
                    input = format!("{input}{},", m);
                }
                input.pop();
                input.push('\n')
            }
            input.push_str("n\n");

            let mut computer = IntcodeComputer::from_file("res/day17.txt");
            computer.override_mem(0, 2);
            let output = computer.execute(input.chars().map(|c| c as isize).rev().collect());

            return *output.last().unwrap();
        }
    }

    panic!("No valid path found");
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

fn get_funcs(
    path: &[Move],
    functions: &[Vec<Move>],
    routine: &[usize],
) -> Option<(Vec<Vec<Move>>, Vec<usize>)> {
    if path.is_empty() {
        return Some((functions.to_vec(), routine.to_vec()));
    }

    for (f, function) in functions.iter().enumerate() {
        if let Some(remainder) = path.strip_prefix(function.as_slice()) {
            let mut routine = routine.to_vec();
            routine.push(f);

            if let Some(solution) = get_funcs(remainder, functions, &routine) {
                return Some(solution);
            }
        }
    }

    if functions.len() < 3 {
        for i in 2..=std::cmp::min(20, path.len()) {
            let mut functions = functions.to_vec();
            functions.push(path[0..i].to_vec());

            let mut routine = routine.to_vec();
            routine.push(functions.len() - 1);

            if let Some(solution) = get_funcs(&path[i..], &functions, &routine) {
                return Some(solution);
            }
        }
    }

    None
}

type Point = (isize, isize);

/// (Start, Scaffolds, Intersections)
fn parse(from: &str) -> (State, HashSet<Point>, HashSet<Point>) {
    let mut scaffolds = HashSet::new();
    let mut start = None;

    let mut i = 0;
    let mut j = 0;

    for t in from.chars() {
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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

impl From<char> for Move {
    fn from(from: char) -> Self {
        match from {
            'L' => Self::Left,
            'R' => Self::Right,
            d if from.is_numeric() => Self::Forward(d.to_digit(10).unwrap() as usize),
            _ => panic!("Can't parse `{}` as a move", from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fmt_path(path: &[Move]) -> String {
        let mut ret = String::with_capacity(path.len());

        for mov in path.iter() {
            ret.push_str(&format!("{}", mov));
        }

        ret
    }

    #[test]
    fn examples() {
        let input = r#"#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......"#;

        let input = parse(input);

        assert!(find_path(input.0, input.1, &input.2)
            .into_iter()
            .map(|p| fmt_path(&trim_path(p)))
            .any(|p| &p == "R8R8R4R4R8L6L2R4R4R8R8R8L6L2"));

        let path: Vec<Move> = "R8R8R4R4R8L6L2R4R4R8R8R8L6L2"
            .chars()
            .map(Move::from)
            .collect();

        assert!(get_funcs(&path, &[], &[]).is_some());
    }
}
