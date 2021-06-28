#[derive(Clone)]
pub struct IntcodeComputer {
  mem: Vec<isize>,
}

impl IntcodeComputer {
  pub fn new(mem: &Vec<isize>) -> Self {
    Self { mem: mem.clone() }
  }

  pub fn execute(&mut self, mut input: Vec<isize>) -> Vec<isize> {
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
          self.mem[target] = input.pop().expect("No inputs available");
          i += 2;
        }
        4 => {
          let param1 = self.get_param(mode_1, self.mem[i + 1]);
          ret.push(param1);
          i += 2;
        }
        5 => {
          let param1 = self.get_param(mode_1, self.mem[i + 1]);
          if param1 != 0 {
            let param2 = self.get_param(mode_2, self.mem[i + 2]);

            assert!(
              param2 >= 0,
              "Encountered negative jump destination ({}) at index {}",
              param2,
              i + 2
            );
            i = param2 as usize;
          } else {
            i += 3;
          }
        }
        6 => {
          let param1 = self.get_param(mode_1, self.mem[i + 1]);
          if param1 == 0 {
            let param2 = self.get_param(mode_2, self.mem[i + 2]);

            assert!(
              param2 >= 0,
              "Encountered negative jump destination ({}) at index {}",
              param2,
              i + 2
            );
            i = param2 as usize;
          } else {
            i += 3;
          }
        }
        7 => {
          let param1 = self.get_param(mode_1, self.mem[i + 1]);
          let param2 = self.get_param(mode_2, self.mem[i + 2]);
          let target = self.mem[i + 3] as usize;
          self.mem[target] = (param1 < param2) as isize;
          i += 4;
        }
        8 => {
          let param1 = self.get_param(mode_1, self.mem[i + 1]);
          let param2 = self.get_param(mode_2, self.mem[i + 2]);
          let target = self.mem[i + 3] as usize;
          self.mem[target] = (param1 == param2) as isize;
          i += 4;
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
