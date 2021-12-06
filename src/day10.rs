use crate::util::gcd;
use std::{
    cmp::{max, min},
    collections::{BTreeMap, HashSet},
    fs::File,
    io::Read,
};

const ERROR_DIGITS: i32 = 3;

pub fn part1() -> usize {
    let asteroids = parse();

    find_laser(&asteroids).0
}

pub fn part2() -> usize {
    let asteroids = parse();

    let laser = find_laser(&asteroids).1;

    let mut polars: BTreeMap<u64, BTreeMap<u64, Point>> = BTreeMap::new();

    for asteroid in asteroids.into_iter() {
        if asteroid == laser {
            continue;
        }

        let polar = asteroid.polar(&laser);

        let (radius, angle) = (
            (polar.0 * 10_f64.powi(ERROR_DIGITS)) as u64,
            (polar.1 * 10_f64.powi(ERROR_DIGITS)) as u64,
        );

        if let Some(line) = polars.get_mut(&angle) {
            assert!(line.insert(radius, asteroid).is_none());
        } else {
            let mut line = BTreeMap::new();
            line.insert(radius, asteroid);
            polars.insert(angle, line);
        }
    }

    let mut cnt = 0;

    while !polars.is_empty() {
        for line in polars.values_mut() {
            let asteroid = line.pop_first().unwrap().1;

            cnt += 1;

            if cnt == 200 {
                return asteroid.j * 100 + asteroid.i;
            }
        }

        polars.retain(|_, line| !line.is_empty());
    }

    panic!("Could not find 200th asteroid");
}

fn find_laser(asteroids: &HashSet<Point>) -> (usize, Point) {
    let mut best = (0, Point::default());

    for asteroid in asteroids.iter() {
        let mut cnt = 0;

        for other in asteroids.iter() {
            if asteroid == other {
                continue;
            }

            if asteroid.visible_from(other, asteroids) {
                cnt += 1;
            }
        }

        if best.0 < cnt {
            best = (cnt, asteroid.clone());
        }
    }

    best
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
struct Point {
    i: usize,
    j: usize,
}

impl Point {
    pub fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    pub fn polar(&self, from: &Self) -> (f64, f64) {
        let di = self.i as isize - from.i as isize;
        let dj = self.j as isize - from.j as isize;

        let radius = (di.pow(2) as f64 + dj.pow(2) as f64).sqrt();

        let mut angle = if radius == 0.0 {
            0.0
        } else {
            (di as f64).atan2(dj as f64).to_degrees() + 90.0
        };

        if angle < 0.0 {
            angle += 360.0;
        }

        (radius, angle)
    }

    pub fn shifted(&self, i: isize, j: isize) -> Self {
        Self {
            i: (i + self.i as isize) as usize,
            j: (j + self.j as isize) as usize,
        }
    }

    pub fn visible_from(&self, from: &Self, obstacles: &HashSet<Point>) -> bool {
        let d_i = from.i as isize - self.i as isize;
        let d_j = from.j as isize - self.j as isize;

        let gcd = gcd(max(d_i.abs(), d_j.abs()), min(d_i.abs(), d_j.abs()));

        let step_i = d_i / gcd;
        let step_j = d_j / gcd;

        let mut intermediary = self.shifted(step_i, step_j);

        while intermediary != *from {
            if obstacles.contains(&intermediary) {
                return false;
            }

            intermediary = intermediary.shifted(step_i, step_j);
        }

        true
    }
}

fn parse() -> HashSet<Point> {
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

    for (i, line) in map.into_iter().enumerate() {
        for (j, character) in line.into_iter().enumerate() {
            if character == '.' {
                continue;
            }

            assert_eq!(character, '#');

            asteroids.insert(Point::new(i, j));
        }
    }

    asteroids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polar() {
        let center = Point::new(5, 5);

        assert_eq!(Point::new(5, 5).polar(&center), (0_f64.sqrt(), 0.0));
        assert_eq!(Point::new(4, 5).polar(&center), (1_f64.sqrt(), 0.0));
        assert_eq!(Point::new(4, 6).polar(&center), (2_f64.sqrt(), 45.0));
        assert_eq!(Point::new(5, 6).polar(&center), (1_f64.sqrt(), 90.0));
        assert_eq!(Point::new(6, 6).polar(&center), (2_f64.sqrt(), 135.0));
        assert_eq!(Point::new(6, 5).polar(&center), (1_f64.sqrt(), 180.0));
        assert_eq!(Point::new(6, 4).polar(&center), (2_f64.sqrt(), 225.0));
        assert_eq!(Point::new(5, 4).polar(&center), (1_f64.sqrt(), 270.0));
        assert_eq!(Point::new(4, 4).polar(&center), (2_f64.sqrt(), 315.0));
    }

    #[test]
    fn visible_from() {
        let a = Point::new(0, 0);
        let b = Point::new(3, 3);

        let others = vec![Point::new(1, 1)].into_iter().collect();
        assert!(!a.visible_from(&b, &others));
        assert!(!b.visible_from(&a, &others));

        let others = vec![Point::new(2, 2)].into_iter().collect();
        assert!(!a.visible_from(&b, &others));
        assert!(!b.visible_from(&a, &others));

        let others = vec![Point::new(2, 1)].into_iter().collect();
        assert!(a.visible_from(&b, &others));
        assert!(b.visible_from(&a, &others));
    }
}
