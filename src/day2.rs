use std::fs::File;
use std::io::Read;

pub fn part1() -> usize {
  let mut data = parse();

  data[1] = 12;
  data[2] = 2;

  execute(data)
}

pub fn part2() -> usize {
  let initial_data = parse();

  for noun in 0..99 {
    for verb in 0..99 {
      let mut data = initial_data.clone();

      data[1] = noun;
      data[2] = verb;

      if execute(data) == 19690720 {
        return 100 * noun + verb;
      }
    }
  }

  panic!("Could not find solution to part 2");
}

fn execute(mut data: Vec<usize>) -> usize {
  let mut i = 0;

  loop {
    match data[i] {
      1 => {
        let target = data[i + 3];
        data[target] = data[data[i + 1]] + data[data[i + 2]];
      }
      2 => {
        let target = data[i + 3];
        data[target] = data[data[i + 1]] * data[data[i + 2]];
      }
      99 => break,
      _ => panic!("Illegal op code ({}) encountered", data[i]),
    }
    i += 4;
  }

  data[0]
}

fn parse() -> Vec<usize> {
  let mut content = String::new();

  File::open("res/day2.txt")
    .unwrap()
    .read_to_string(&mut content)
    .unwrap();

  content
    .split(",")
    .map(|s| s.trim())
    .filter_map(|s| s.parse().ok())
    .collect()
}
