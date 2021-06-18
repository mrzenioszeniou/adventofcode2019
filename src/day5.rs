use std::{fs::File, io::Read};

pub fn part1() -> isize {
  let mut computer = parse();

  let output = computer.execute();

  let non_zero: Vec<isize> = output.into_iter().skip_while(|n| *n == 0).collect();

  assert_eq!(non_zero.len(), 1);

  non_zero[0].to_owned()
}

pub fn part2() -> usize {
  42
}

pub struct IntcodeComputer {
  mem: Vec<isize>,
}

impl IntcodeComputer {
  pub fn new(mem: &Vec<isize>) -> Self {
    Self { mem: mem.clone() }
  }

  // First, you'll need to add two new instructions:
  // Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50.
  // Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.
  pub fn execute(&mut self) -> Vec<isize> {
    let mut i = 0;

    let mut ret = vec![];

    loop {
      let (op, mode_1, mode_2, _) = self.parse_intcode(i);

      match op {
        1 => {
          let param1 = self.get_param(mode_1, self.mem[i + 1]);
          let param2 = self.get_param(mode_2, self.mem[i + 2]);
          let target = self.mem[i + 3] as usize;
          self.mem[target] = param1 + param2;
          i += 4;
        }
        2 => {
          let param1 = self.get_param(mode_1, self.mem[i + 1]);
          let param2 = self.get_param(mode_2, self.mem[i + 2]);
          let target = self.mem[i + 3] as usize;
          self.mem[target] = param1 * param2;
          i += 4;
        }
        3 => {
          let target = self.mem[i + 1] as usize;
          self.mem[target] = 1;
          i += 2;
        }
        4 => {
          let param1 = self.get_param(mode_1, self.mem[i + 1]);
          ret.push(param1);
          i += 2;
        }
        99 => break,
        _ => panic!(
          "Illegal op code ({}) encountered at index {}",
          self.mem[i], i
        ),
      }
    }

    ret
  }

  fn get_param(&self, mode: usize, param: isize) -> isize {
    match mode {
      0 => {
        assert!(param >= 0);
        self.mem[param as usize]
      }
      1 => param,
      _ => panic!("Illegal parameter mode ({}) encountered", mode),
    }
  }

  fn parse_intcode(&self, index: usize) -> (usize, usize, usize, usize) {
    assert!(self.mem[index] >= 0, "Found negative opcode");

    let value = self.mem[index] as usize;

    let op_code = value % 100;

    let mode_1 = value / 100 % 10;
    let mode_2 = value / 1000 % 10;
    let mode_3 = value / 10000 % 10;

    (op_code, mode_1, mode_2, mode_3)
  }
}

fn parse() -> IntcodeComputer {
  let mut content = String::new();

  File::open("res/day5.txt")
    .unwrap()
    .read_to_string(&mut content)
    .unwrap();

  let memory = content
    .trim()
    .split(",")
    .map(|s| s.parse::<isize>().unwrap())
    .collect();

  IntcodeComputer::new(&memory)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_computer() {
    let mut computer = IntcodeComputer::new(&vec![1002, 4, 3, 4, 33]);

    let results = computer.execute();

    assert!(results.is_empty());
    assert_eq!(computer.mem, vec![1002, 4, 3, 4, 99]);
  }
}
