use std::collections::HashMap;

use crate::comp::IntcodeComputer;

pub fn part1() -> usize {
  let mut arcade = IntcodeComputer::from_file("res/day13.txt");

  let output = arcade.execute(vec![]);

  let mut tiles: HashMap<(isize, isize), isize> = HashMap::new();

  assert!(arcade.is_done());
  assert_eq!(output.len() % 3, 0);

  for i in (0..output.len()).step_by(3) {
    tiles.insert((output[i], output[i + 1]), output[i + 2]);
  }

  tiles.values().filter(|tile| **tile == 2).count()
}

pub fn part2() -> isize {
  let mut arcade = IntcodeComputer::from_file("res/day13.txt");
  arcade.override_mem(0, 2);
  let mut ball = (0, 0);
  let mut first = true;
  let mut paddle = (0, 0);
  let mut score = 0;
  let mut screen = vec![vec![' '; 100]; 100];

  while !arcade.is_done() {
    let input = if first {
      first = false;
      vec![]
    } else if ball.1 < paddle.1 {
      vec![-1]
    } else if ball.1 > paddle.1 {
      vec![1]
    } else {
      vec![0]
    };

    let output = arcade.execute(input);

    for i in (0..output.len()).step_by(3) {
      if output[i..i + 2] == [-1, 0] {
        score = output[i + 2];
        continue;
      }

      let tile_char = match output[i + 2] {
        0 => ' ',   // Empty
        1 => '█', // Wall
        2 => '▒', // Block
        3 => {
          // Paddle
          paddle = (output[i + 1], output[i]);
          '═'
        }
        4 => {
          // Ball
          ball = (output[i + 1], output[i]);
          'o'
        }
        _ => panic!("The heck is this?"),
      };

      screen[output[i + 1] as usize][output[i] as usize] = tile_char;
    }

    // println!("SCORE:{:<}", score);
    // for line in screen.iter() {
    //   if line.iter().all(|c| *c == ' ') {
    //     continue;
    //   }

    //   for tile in line.iter() {
    //     print!("{}", tile);
    //   }
    //   println!();
    // }
    // std::thread::sleep(std::time::Duration::from_millis(10));
  }

  score
}
