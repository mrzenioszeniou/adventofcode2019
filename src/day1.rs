use std::fs::File;
use std::io::Read;

pub fn part1() -> usize {
    parse().iter().map(|w| calc_fuel(*w)).sum()
}

pub fn part2() -> usize {
    parse().iter().map(|w| calc_fuel_recursive(*w)).sum()
}

fn parse() -> Vec<usize> {
    let mut content = String::new();

    File::open("res/day1.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    content
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn calc_fuel(mass: usize) -> usize {
    mass / 3 - 2
}

fn calc_fuel_recursive(mass: usize) -> usize {
    if mass < 6 {
        0
    } else {
        let fuel = calc_fuel(mass);
        fuel + calc_fuel_recursive(fuel)
    }
}
