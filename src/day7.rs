use std::cmp::max;

use crate::comb::permutations;
use crate::comp::IntcodeComputer;

pub fn part1() -> usize {
    let base = IntcodeComputer::from_file("res/day7.txt");

    let mut best = 0;

    for (_i, phase_settings) in permutations(&[0, 1, 2, 3, 4]).into_iter().enumerate() {
        let out_a = base.clone().execute(vec![0, phase_settings[0]]);
        assert_eq!(out_a.len(), 1);

        let out_b = base.clone().execute(vec![out_a[0], phase_settings[1]]);
        assert_eq!(out_b.len(), 1);

        let out_c = base.clone().execute(vec![out_b[0], phase_settings[2]]);
        assert_eq!(out_c.len(), 1);

        let out_d = base.clone().execute(vec![out_c[0], phase_settings[3]]);
        assert_eq!(out_d.len(), 1);

        let out_e = base.clone().execute(vec![out_d[0], phase_settings[4]]);
        assert_eq!(out_e.len(), 1);

        best = max(best, out_e[0]);
    }

    best as usize
}

pub fn part2() -> usize {
    let base = IntcodeComputer::from_file("res/day7.txt");

    let mut best = 0;

    for (_i, phase_settings) in permutations(&[5, 6, 7, 8, 9]).into_iter().enumerate() {
        let mut amps = vec![base.clone(); 5];

        for (amp, phase) in amps.iter_mut().zip(phase_settings) {
            assert!(!amp.is_done());
            let out = amp.execute(vec![phase]);
            assert!(out.is_empty());
        }

        let mut prev = 0;

        while amps.iter().any(|amp| !amp.is_done()) {
            for amp in amps.iter_mut() {
                assert!(!amp.is_done());

                let out = amp.execute(vec![prev]);
                assert_eq!(out.len(), 1);
                prev = out[0];
            }
        }

        best = max(best, prev);
    }

    best as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        assert_eq!(part1(), 34852);
        assert_eq!(part2(), 44282086);
    }
}
