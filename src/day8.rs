use std::{fs::File, io::Read};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn part1() -> usize {
  let layers = parse();

  let mut answer = None;

  for layer in layers.iter() {
    let mut zeros = 0;
    let mut ones = 0;
    let mut twos = 0;

    for n in layer.iter().flat_map(|v| v.iter()) {
      match n {
        '0' => zeros += 1,
        '1' => ones += 1,
        '2' => twos += 1,
        _ => panic!("Unexpected digits found ({})", n),
      }
    }

    match answer {
      None => answer = Some((zeros, ones * twos)),
      Some((prev_zeros, _)) if prev_zeros > zeros => answer = Some((zeros, ones * twos)),
      _ => {}
    }
  }

  answer.unwrap().1
}

pub fn part2() -> String {
  let mut ret = String::new();

  let layers = parse();

  ret.push('\n');

  for i in 0..HEIGHT {
    for j in 0..WIDTH {
      for layer in layers.iter() {
        match layer[i][j] {
          '0' => {
            ret.push('█');
            break;
          }
          '1' => {
            ret.push(' ');
            break;
          }
          '2' => continue,
          _ => panic!("Unexpected digits found ({})", layer[i][j]),
        }
      }
    }
    ret.push('\n');
  }

  ret
}

type Layer = Vec<Vec<char>>;

fn parse() -> Vec<Layer> {
  let mut content = String::with_capacity(HEIGHT * (WIDTH + 1) + 1);

  File::open("res/day8.txt")
    .unwrap()
    .read_to_string(&mut content)
    .unwrap();

  let mut layers = vec![];
  let mut layer = vec![];
  let mut line = vec![];

  for c in content.chars() {
    line.push(c);

    if line.len() == WIDTH {
      layer.push(line);
      line = vec![];
      if layer.len() == HEIGHT {
        layers.push(layer);
        layer = vec![];
      }
    }
  }

  layers
}
