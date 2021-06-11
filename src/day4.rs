pub fn part1() -> usize {
  let mut cnt = 0;

  let mut counter = Counter::new(206938, 679128);

  while !counter.done() {
    if counter.check_and_inc1() {
      cnt += 1;
    }
  }

  cnt
}

pub fn part2() -> usize {
  let mut cnt = 0;

  let mut counter = Counter::new(206938, 679128);

  while !counter.done() {
    if counter.check_and_inc2() {
      cnt += 1;
    }
  }

  cnt
}

struct Counter {
  digits: Vec<u8>,
  target: Vec<u8>,
}

impl Counter {
  fn num_to_vec(mut from: usize) -> Vec<u8> {
    let mut digits = vec![];

    while from > 0 {
      digits.push((from % 10) as u8);
      from /= 10;
    }

    digits.reverse();

    digits
  }

  pub fn new(from: usize, to: usize) -> Self {
    let digits = Self::num_to_vec(from);
    let target = Self::num_to_vec(to);

    assert_eq!(digits.len(), target.len());

    Self { digits, target }
  }

  pub fn check_and_inc1(&mut self) -> bool {
    let mut repeating = false;

    for i in 0..self.digits.len() - 1 {
      if self.digits[i] == self.digits[i + 1] {
        repeating = true;
      } else if self.digits[i] > self.digits[i + 1] {
        for j in i + 1..self.digits.len() {
          self.digits[j] = self.digits[i];
        }
        return false;
      }
    }

    self.inc(self.digits.len() - 1);

    repeating
  }
  pub fn check_and_inc2(&mut self) -> bool {
    let mut is_repeat = false;
    let mut repetitions = 1;

    for i in 0..self.digits.len() - 1 {
      if self.digits[i] == self.digits[i + 1] {
        repetitions += 1;
      } else {
        if repetitions == 2 {
          is_repeat = true;
        }
        repetitions = 1;
      }

      if self.digits[i] > self.digits[i + 1] {
        for j in i + 1..self.digits.len() {
          self.digits[j] = self.digits[i];
        }
        return false;
      }
    }

    self.inc(self.digits.len() - 1);

    is_repeat || repetitions == 2
  }

  fn inc(&mut self, mut i: usize) {
    self.digits[i] += 1;

    while self.digits[i] > 9 {
      self.digits[i] = 0;
      i -= 1;
      self.digits[i] += 1;
    }
  }

  pub fn done(&self) -> bool {
    for i in 0..self.digits.len() {
      if self.digits[i] > self.target[i] {
        return true;
      } else if self.digits[i] < self.target[i] {
        return false;
      }
    }

    false
  }
}
