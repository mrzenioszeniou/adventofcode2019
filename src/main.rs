#![feature(map_first_last)]
#![feature(int_roundings)]

mod comb;
mod comp;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod dir;
mod util;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: adventofcode2019 DAY");
        std::process::exit(1)
    }

    let day: usize = args[1].parse().expect("Couldn't parse day");

    match day {
        1 => println!("PART 1:{}\nPART 2:{}", day1::part1(), day1::part2()),
        2 => println!("PART 1:{}\nPART 2:{}", day2::part1(), day2::part2()),
        3 => println!("PART 1:{}\nPART 2:{}", day3::part1(), day3::part2()),
        4 => println!("PART 1:{}\nPART 2:{}", day4::part1(), day4::part2()),
        5 => println!("PART 1:{}\nPART 2:{}", day5::part1(), day5::part2()),
        6 => println!("PART 1:{}\nPART 2:{}", day6::part1(), day6::part2()),
        7 => println!("PART 1:{}\nPART 2:{}", day7::part1(), day7::part2()),
        8 => println!("PART 1:{}\nPART 2:{}", day8::part1(), day8::part2()),
        9 => println!("PART 1:{}\nPART 2:{}", day9::part1(), day9::part2()),
        10 => println!("PART 1:{}\nPART 2:{}", day10::part1(), day10::part2()),
        11 => println!("PART 1:{}\nPART 2:{}", day11::part1(), day11::part2()),
        12 => println!("PART 1:{}\nPART 2:{}", day12::part1(), day12::part2()),
        13 => println!("PART 1:{}\nPART 2:{}", day13::part1(), day13::part2()),
        14 => println!("PART 1:{}\nPART 2:{}", day14::part1(), day14::part2()),
        _ => println!("No implementation available for day {}", day),
    }
}
