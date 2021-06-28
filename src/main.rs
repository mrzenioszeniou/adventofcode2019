extern crate regex;

mod comb;
mod comp;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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
    _ => println!("No implementation available for day {}", day),
  }
}
