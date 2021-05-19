mod day1;

fn main() {

  let args:Vec<String> = std::env::args().collect();

  if args.len() != 2 {
    println!("Usage: adventofcode2019 DAY");
    std::process::exit(1)
  }

  let day:usize = args[1].parse().expect("Couldn't parse day");


  match day {
    1 => println!("PART 1:{}\nPART 2:{}", day1::part1(), day1::part2()),
    _ => println!("No implementation available for day {}", day),
  }
}
