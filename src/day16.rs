const PATTERN: [isize; 4] = [0, 1, 0, -1];

pub fn part1() -> isize {
    let mut signal = parse();

    for _ in 0..100 {
        let mut next = vec![];

        for i in 0..signal.len() {
            let mut digit = 0;
            for j in 0..signal.len() {
                digit += signal[j] * PATTERN[(j + 1) / (i + 1) % PATTERN.len()];
            }
            next.push(digit.abs() % 10);
        }

        signal = next;
    }

    signal[0..8]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| d * 10_isize.pow(i as u32))
        .sum()
}

pub fn part2() -> isize {
    let mut signal = parse().repeat(10_000);

    let offset: usize = signal[0..7]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| *d as usize * 10_usize.pow(i as u32))
        .sum();

    for step in 0..100 {
        let mut next = vec![];

        for i in 0..signal.len() {
            let mut digit = 0;
            for j in i..signal.len() {
                digit += signal[j] * PATTERN[(j + 1) / (i + 1) % PATTERN.len()];
            }
            next.push(digit.abs() % 10);
            println!("({},{})", step, i);
        }

        signal = next;

        println!("{}", step);
    }

    signal[offset..offset + 8]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| d * 10_isize.pow(i as u32))
        .sum()
}

fn phase(num: &[isize]) -> Vec<isize> {
    let mut ret = vec![0; num.len()];

    for i in 0..num.len() {
        println!("{}", i);
        for j in 0..num.len() {
            ret[i] += num[j] * PATTERN[(j + 1) / (i + 1) % PATTERN.len()];
        }
    }

    ret.into_iter().map(|n| n.abs() % 10).collect()
}

fn parse() -> Vec<isize> {
    std::fs::read_to_string("res/day16.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut signal = vec![1, 2, 3, 4, 5, 6, 7, 8];

        signal = phase(&signal);

        assert_eq!(signal, vec![4, 8, 2, 2, 6, 1, 5, 8]);

        signal = phase(&signal);

        assert_eq!(signal, vec![3, 4, 0, 4, 0, 4, 3, 8]);

        signal = phase(&signal);

        assert_eq!(signal, vec![0, 3, 4, 1, 5, 5, 1, 8]);

        signal = phase(&signal);

        assert_eq!(signal, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }
}
