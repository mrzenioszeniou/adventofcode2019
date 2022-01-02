const PATTERN: [isize; 4] = [0, 1, 0, -1];

pub fn part1() -> isize {
    let mut signal = parse();

    for _ in 0..100 {
        let mut next = vec![];

        for i in 0..signal.iter().len() {
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

    for _ in 0..100 {
        let mut prev = *signal.last().unwrap();

        for digit in signal.iter_mut().skip(offset).rev() {
            *digit = (*digit + prev) % 10;
            prev = *digit;
        }
    }

    signal[offset..offset + 8]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| d * 10_isize.pow(i as u32))
        .sum()
}

fn parse() -> Vec<isize> {
    std::fs::read_to_string("res/day16.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect()
}
