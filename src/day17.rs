use std::{
    collections::{BTreeSet, HashSet},
    convert::TryFrom,
    iter::FromIterator,
};

use crate::{
    comp::IntcodeComputer,
    dir::{neighbours, Direction},
};

pub fn part1() -> isize {
    parse().1.iter().map(|(i, j)| i * j).sum()
}

pub fn part2() -> usize {
    let (start, intersections) = parse();

    let paths = find_path(start, HashSet::new(), &intersections);

    assert!(!paths.is_empty());

    println!("{} paths found", paths.len());

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

    paths.len()
}

fn find_path(
    state: State,
    visited: HashSet<Point>,
    intersections: &HashSet<Point>,
) -> HashSet<Vec<State>> {
    if state.unvisited.is_empty() {
        return HashSet::from([vec![state]]);
    }

    let mut ret = HashSet::new();

    for (next, next_dir) in neighbours(state.pos) {
        if state.unvisited.contains(&next)
            || intersections.contains(&next) && !visited.contains(&next)
        {
            let mut visited = visited.clone();

            if !state.unvisited.contains(&next) {
                visited.insert(next);
            }

            let mut next_state = State {
                pos: next,
                dir: next_dir,
                unvisited: state.unvisited.clone(),
            };
            next_state.unvisited.remove(&next);

            for mut path in find_path(next_state, visited, intersections) {
                path.push(state.clone());
                ret.insert(path);
            }
        }
    }

    ret
}

type Point = (isize, isize);

/// (Start, Scaffolds, Intersections)
fn parse() -> (State, HashSet<Point>) {
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
                scaffolds.insert((i, j));
                start = Some(State {
                    pos: (i, j),
                    dir: Direction::try_from(t).unwrap(),
                    unvisited: BTreeSet::new(),
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

    let mut start = start.unwrap();
    start.unvisited = BTreeSet::from_iter(scaffolds.into_iter());
    start.unvisited.remove(&start.pos);

    (start, intersections)
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    pos: Point,
    dir: Direction,
    unvisited: BTreeSet<Point>,
}
