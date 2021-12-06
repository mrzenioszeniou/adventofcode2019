use std::{
    cmp::Ordering,
    fs::File,
    hash::Hash,
    io::Read,
    ops::{AddAssign, SubAssign},
};

use crate::util::lcm_many;

pub fn part1() -> isize {
    let mut moons = parse();

    for _ in 0..1000 {
        accelerate(&mut moons);
        moons.iter_mut().for_each(Moon::step);
    }

    moons.iter().map(Moon::energy).sum()
}

pub fn part2() -> usize {
    let initial = parse();
    let mut moons = initial.clone();

    let mut x_period = None;
    let mut y_period = None;
    let mut z_period = None;
    let mut i = 0;

    while x_period.is_none() || y_period.is_none() || z_period.is_none() {
        accelerate(&mut moons);
        moons.iter_mut().for_each(Moon::step);
        i += 1;

        let mut eq_x = true;
        let mut eq_y = true;
        let mut eq_z = true;

        for (moon, init) in moons.iter().zip(&initial) {
            if x_period.is_none() && !moon.eq_x(init) {
                eq_x = false;
            }

            if y_period.is_none() && !moon.eq_y(init) {
                eq_y = false;
            }

            if z_period.is_none() && !moon.eq_z(init) {
                eq_z = false;
            }
        }

        if x_period.is_none() && eq_x {
            x_period = Some(i);
        }

        if y_period.is_none() && eq_y {
            y_period = Some(i);
        }

        if z_period.is_none() && eq_z {
            z_period = Some(i);
        }
    }

    println!(
        "<x={},y={},z={}>",
        x_period.unwrap(),
        y_period.unwrap(),
        z_period.unwrap()
    );

    lcm_many(&[x_period.unwrap(), y_period.unwrap(), z_period.unwrap()])
}

fn parse() -> Vec<Moon> {
    let mut content = String::new();

    File::open("res/day12.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let mut ret: Vec<Moon> = vec![];

    for line in content.split('\n') {
        let nums: Vec<_> = line
            .split_ascii_whitespace()
            .map(|dim| dim.parse().unwrap())
            .collect();

        assert_eq!(nums.len(), 3);

        ret.push(Moon::new(Vector {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        }));
    }

    ret
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Moon {
    position: Vector,
    speed: Vector,
}

impl Moon {
    pub fn new(position: Vector) -> Self {
        Self {
            position,
            speed: Vector::default(),
        }
    }

    pub fn step(&mut self) {
        self.position += self.speed;
    }

    pub fn energy(&self) -> isize {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs())
            * (self.speed.x.abs() + self.speed.y.abs() + self.speed.z.abs())
    }

    pub fn eq_x(&self, other: &Self) -> bool {
        self.position.x == other.position.x && self.speed.x == other.speed.x
    }
    pub fn eq_y(&self, other: &Self) -> bool {
        self.position.y == other.position.y && self.speed.y == other.speed.y
    }
    pub fn eq_z(&self, other: &Self) -> bool {
        self.position.z == other.position.z && self.speed.z == other.speed.z
    }
}

fn accelerate(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        let mut force = Vector::default();

        for j in 0..moons.len() {
            if i == j {
                continue;
            }

            match moons[i].position.x.cmp(&moons[j].position.x) {
                Ordering::Less => force.x += 1,
                Ordering::Greater => force.x -= 1,
                Ordering::Equal => {}
            }

            match moons[i].position.y.cmp(&moons[j].position.y) {
                Ordering::Less => force.y += 1,
                Ordering::Greater => force.y -= 1,
                Ordering::Equal => {}
            }

            match moons[i].position.z.cmp(&moons[j].position.z) {
                Ordering::Less => force.z += 1,
                Ordering::Greater => force.z -= 1,
                Ordering::Equal => {}
            }
        }

        moons[i].speed += force;
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
