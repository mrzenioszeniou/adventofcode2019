use crate::comp::IntcodeComputer;

pub fn part1() -> isize {
    let mut computer = IntcodeComputer::from_file("res/day5.txt");

    let output = computer.execute(vec![1]);

    let non_zero: Vec<isize> = output.into_iter().skip_while(|n| *n == 0).collect();

    assert_eq!(non_zero.len(), 1);

    non_zero[0].to_owned()
}

pub fn part2() -> isize {
    let mut computer = IntcodeComputer::from_file("res/day5.txt");

    let output = computer.execute(vec![5]);

    assert_eq!(output.len(), 1);

    output[0].to_owned()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        assert_eq!(part1(), 4887191);
        assert_eq!(part2(), 3419022);
    }
}
