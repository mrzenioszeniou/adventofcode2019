use std::collections::{BTreeSet, HashMap, HashSet};

use crate::{dir::neighbours, util::a_star};

pub fn part1() -> usize {
    let input = std::fs::read_to_string("res/day18.txt").unwrap();
    part1_solver(&input)
}

fn part1_solver(input: &str) -> usize {
    let (tiles, mut keys, doors) = parse(input);

    let keys_and_start = keys.clone();

    keys.remove(&'@').unwrap();

    let nexts = |pos: &Point| {
        neighbours((pos.0 as isize, pos.1 as isize))
            .into_iter()
            .map(|(n, _)| (n, 1))
            .filter(|(n, _)| tiles.contains(n))
            .collect()
    };

    let mut paths: HashMap<(char, char), Vec<Point>> = HashMap::new();
    let mut dependencies: HashMap<(char, char), HashSet<char>> = HashMap::new();

    for from in keys_and_start.iter() {
        for to in keys.iter() {
            if from == to {
                continue;
            }

            if let Some((mut path, _)) = a_star(*from.1, *to.1, nexts, |p| {
                p.0.abs_diff(to.1 .0) + p.1.abs_diff(to.1 .1)
            }) {
                path.remove(0);

                dependencies.entry((*from.0, *to.0)).or_default().extend(
                    path.iter()
                        .filter_map(|p| doors.get(p).map(|d| d.to_ascii_lowercase())),
                );

                paths.insert((*from.0, *to.0), path);
            } else {
                panic!("Can't get from `{}` to `{}`", from.0, to.0);
            }
        }
    }

    // println!("Paths:");
    // for (pair, path) in paths.iter() {
    //     println!(
    //         "{}→{}: {} steps dependent on {:?}",
    //         pair.0,
    //         pair.1,
    //         path.len() - 1,
    //         dependencies.get(pair).unwrap()
    //     );
    // }

    let mut cache = HashMap::new();

    find_shortest_path(
        '@',
        keys.keys().cloned().collect(),
        &paths,
        &dependencies,
        &mut cache,
    )
    .len()

    // 42

    // solve(start, &tiles, keys, doors)
    //     .expect("No solution found")
    //     .len()
}

fn find_shortest_path(
    curr: char,
    keys: BTreeSet<char>,
    paths: &HashMap<(char, char), Vec<Point>>,
    dependencies: &HashMap<(char, char), HashSet<char>>,
    cache: &mut HashMap<char, HashMap<BTreeSet<char>, Vec<Point>>>,
) -> Vec<Point> {
    // println!("{} @ {:?}", curr, keys);

    if keys.is_empty() {
        return vec![];
    }

    if let Some(ret) = cache.get(&curr).and_then(|k| k.get(&keys)) {
        return ret.clone();
    }

    let mut best: Option<Vec<Point>> = None;

    for to in keys.iter() {
        if dependencies
            .get(&(curr, *to))
            .unwrap()
            .iter()
            .all(|d| !keys.contains(d))
        {
            let mut keys = keys.clone();
            keys.remove(to);

            let subpath = find_shortest_path(*to, keys, paths, dependencies, cache);
            let path = paths.get(&(curr, *to)).unwrap();

            if best.as_ref().map(|b| b.len()).unwrap_or(usize::MAX) > subpath.len() + path.len() {
                let mut path = path.clone();
                path.extend(&subpath);
                best = Some(path);
            }
        }
    }

    let ret = best.unwrap();

    if cache
        .get(&curr)
        .and_then(|k| k.get(&keys))
        .map(|p| p.len())
        .unwrap_or(usize::MAX)
        > ret.len()
    {
        cache.entry(curr).or_default().insert(keys, ret.clone());
    }

    ret
}

pub fn part2() -> usize {
    42
}

type Point = (isize, isize);

fn parse(from: &str) -> (HashSet<Point>, HashMap<char, Point>, HashMap<Point, char>) {
    let mut tiles = HashSet::new();
    let mut keys = HashMap::new();
    let mut doors = HashMap::new();

    for (i, line) in from.split('\n').enumerate() {
        for (j, c) in line.chars().enumerate() {
            let point = (i as isize, j as isize);

            match c {
                '#' => {}
                '.' => {
                    tiles.insert(point);
                }
                '@' | 'a'..='z' => {
                    tiles.insert(point);
                    keys.insert(c, point);
                }
                'A'..='Z' => {
                    tiles.insert(point);
                    doors.insert(point, c);
                }
                _ => panic!("Unexpected map tile `{}` at ({},{})", c, i, j),
            }
        }
    }

    (tiles, keys, doors)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn examples() {
        let input = "#########
#b.A.@.a#
#########";
        assert_eq!(part1_solver(input), 8);

        let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        assert_eq!(part1_solver(input), 86);

        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        assert_eq!(part1_solver(input), 132);

        let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        assert_eq!(part1_solver(input), 136);

        let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
        assert_eq!(part1_solver(input), 81);
    }
}
