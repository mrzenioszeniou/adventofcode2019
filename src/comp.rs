use std::{fs::File, io::Read};

const MEMORY_SIZE: usize = 4096;
const MEMORY_DEFAULT: isize = 0;

#[derive(Clone)]
pub struct IntcodeComputer {
    mem: Vec<isize>,
    index: usize,
    done: bool,
    rel_base_offset: isize,
}

impl IntcodeComputer {
    pub fn from_file(path: &str) -> Self {
        let mut content = String::new();

        File::open(path)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();

        let mut memory: Vec<isize> = content
            .trim()
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect();

        while memory.len() < MEMORY_SIZE {
            memory.push(MEMORY_DEFAULT);
        }

        Self {
            index: 0,
            mem: memory,
            done: false,
            rel_base_offset: 0,
        }
    }

    pub fn execute(&mut self, mut input: Vec<isize>) -> Vec<isize> {
        let mut ret = vec![];

        loop {
            let (op, mode_1, mode_2, mode_3) = self.parse_intcode(self.index);

            match op {
                1 => {
                    let param1 = self.get_param(mode_1, self.mem[self.index + 1]);
                    let param2 = self.get_param(mode_2, self.mem[self.index + 2]);
                    self.set_param(mode_3, self.mem[self.index + 3], param1 + param2);
                    self.index += 4;
                }
                2 => {
                    let param1 = self.get_param(mode_1, self.mem[self.index + 1]);
                    let param2 = self.get_param(mode_2, self.mem[self.index + 2]);
                    self.set_param(mode_3, self.mem[self.index + 3], param1 * param2);
                    self.index += 4;
                }
                3 => {
                    if input.is_empty() {
                        return ret;
                    } else {
                        self.set_param(
                            mode_1,
                            self.mem[self.index + 1],
                            input.pop().expect("No inputs available"),
                        );
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
                    self.set_param(mode_3, self.mem[self.index + 3], (param1 < param2) as isize);
                    self.index += 4;
                }
                8 => {
                    let param1 = self.get_param(mode_1, self.mem[self.index + 1]);
                    let param2 = self.get_param(mode_2, self.mem[self.index + 2]);
                    self.set_param(
                        mode_3,
                        self.mem[self.index + 3],
                        (param1 == param2) as isize,
                    );
                    self.index += 4;
                }
                9 => {
                    let param1 = self.get_param(mode_1, self.mem[self.index + 1]);
                    self.rel_base_offset += param1;
                    self.index += 2;
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
            2 => {
                let addr = param + self.rel_base_offset;
                self.mem[addr as usize]
            }
            _ => panic!("Illegal parameter mode ({}) encountered", mode),
        }
    }

    fn set_param(&mut self, mode: usize, addr: isize, value: isize) {
        match mode {
            0 => self.mem[addr as usize] = value,
            2 => self.mem[(addr + self.rel_base_offset) as usize] = value,
            _ => panic!("Immediate (1) mode is not permitted for output paramters"),
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

    pub fn override_mem(&mut self, index: usize, value: isize) {
        self.mem[index] = value;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn relative_base_offset_quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut input = program.clone();

        while input.len() < MEMORY_SIZE {
            input.push(MEMORY_DEFAULT);
        }

        let mut computer = IntcodeComputer {
            index: 0,
            mem: input,
            done: false,
            rel_base_offset: 0,
        };

        assert_eq!(computer.execute(vec![]), program);
    }

    #[test]
    fn relative_base_offset_16_digit_num() {
        let mut input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        while input.len() < MEMORY_SIZE {
            input.push(MEMORY_DEFAULT);
        }

        let mut computer = IntcodeComputer {
            index: 0,
            mem: input,
            done: false,
            rel_base_offset: 0,
        };

        let output = computer.execute(vec![]);

        assert_eq!(output.len(), 1);

        assert!(output[0] >= 1_000_000_000_000_000 && output[0] <= 9_999_999_999_999_999);
    }

    #[test]
    fn relative_base_offset_middle_num() {
        let mut input = vec![104, 1125899906842624, 99];

        while input.len() < MEMORY_SIZE {
            input.push(MEMORY_DEFAULT);
        }

        let mut computer = IntcodeComputer {
            index: 0,
            mem: input,
            done: false,
            rel_base_offset: 0,
        };

        assert_eq!(computer.execute(vec![]), vec![1125899906842624]);
    }
}
