#[derive(Clone)]
pub struct IntcodeComputer {
  mem: Vec<isize>,
  index: usize,
  done: bool,
}

impl IntcodeComputer {
  pub fn new(mem: &Vec<isize>) -> Self {
    Self {
      index: 0,
      mem: mem.clone(),
      done: false,
    }
  }

  pub fn execute(&mut self, mut input: Vec<isize>) -> Vec<isize> {
    let mut ret = vec![];

    loop {
      let (op, mode_1, mode_2, _) = self.parse_intcode(self.index);

      match op {
        1 => {
          let param1 = self.get_param(mode_1, self.mem[self.index + 1]);
          let param2 = self.get_param(mode_2, self.mem[self.index + 2]);
          let target = self.mem[self.index + 3] as usize;
          self.mem[target] = param1 + param2;
          self.index += 4;
        }
        2 => {
          let param1 = self.get_param(mode_1, self.mem[self.index + 1]);
          let param2 = self.get_param(mode_2, self.mem[self.index + 2]);
          let target = self.mem[self.index + 3] as usize;
          self.mem[target] = param1 * param2;
          self.index += 4;
        }
        3 => {
          let target = self.mem[self.index + 1] as usize;
          if input.is_empty() {
            return ret;
          } else {
            self.mem[target] = input.pop().expect("No inputs available");
            self.index += 2;
          }
        }
        4 => {
          let param1 = self.get_param(mode_1, self.mem[self.index + 1]);
          ret.push(param1);
          self.index += 2;
        }
        5 => {
          let param1 = self.get_param(mode_1, self.mem[self.index + 1]);
          if param1 != 0 {
            let param2 = self.get_param(mode_2, self.mem[self.index + 2]);

            assert!(
              param2 >= 0,
              "Encountered negative jump destination ({}) at index {}",
              param2,
              self.index + 2
            );
            self.index = param2 as usize;
          } else {
            self.index += 3;
          }
        }
        6 => {
          let param1 = self.get_param(mode_1, self.mem[self.index + 1]);
          if param1 == 0 {
            let param2 = self.get_param(mode_2, self.mem[self.index + 2]);

            assert!(
              param2 >= 0,
              "Encountered negative jump destination ({}) at index {}",
              param2,
              self.index + 2
            );
            self.index = param2 as usize;
          } else {
            self.index += 3;
          }
        }
        7 => {
          let param1 = self.get_param(mode_1, self.mem[self.index + 1]);
          let param2 = self.get_param(mode_2, self.mem[self.index + 2]);
          let target = self.mem[self.index + 3] as usize;
          self.mem[target] = (param1 < param2) as isize;
          self.index += 4;
        }
        8 => {
          let param1 = self.get_param(mode_1, self.mem[self.index + 1]);
          let param2 = self.get_param(mode_2, self.mem[self.index + 2]);
          let target = self.mem[self.index + 3] as usize;
          self.mem[target] = (param1 == param2) as isize;
          self.index += 4;
        }
        99 => {
          self.done = true;
          break;
        }
        _ => panic!(
          "Illegal op code ({}) encountered at index {}",
          self.mem[self.index], self.index
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

  pub fn is_done(&self) -> bool {
    self.done
  }
}
