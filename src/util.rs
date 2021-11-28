pub fn gcd(a: isize, b: isize) -> isize {
  if b == 0 {
    return a;
  }

  gcd(b, a % b)
}

pub fn lcm(a: usize, b: usize) -> usize {
  a * b / (gcd(a as isize, b as isize) as usize)
}

pub fn lcm_many(nums: &[usize]) -> usize {
  if nums.len() < 2 {
    panic!("Requested the least common multiple of less than 2 numbers");
  }

  if nums.len() == 2 {
    return lcm(nums[0], nums[1]);
  }

  lcm(nums[0], lcm_many(&nums[1..]))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(lcm_many(&[3, 4, 6]), 12);
    assert_eq!(lcm_many(&[3, 4, 5, 6]), 60);
  }
}
