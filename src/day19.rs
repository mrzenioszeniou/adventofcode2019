use std::collections::VecDeque;

use crate::comp::IntcodeComputer;

pub fn part1() -> u32 {
    let mut cnt = 0;

    for y in 0..50 {
        for x in 0..50 {
            let res =
                IntcodeComputer::from_file("res/day19.txt").execute(vec![x as isize, y as isize]);

            match &res[..] {
                &[0] => {
                    // print!(".");
                }
                &[1] => {
                    // print!("#");
                    cnt += 1;
                }
                other => panic!("Expected 0 or 1 but got {:?}", other),
            }
        }

        // println!();
    }

    // println!();

    cnt
}

pub fn part2() -> usize {
    const BOX_SIZE: usize = 100;
    let mut hist = VecDeque::default();

    for y in 0.. {
        let mut start = None;
        let mut end = None;

        let prev_start = hist.front().map(|(start, _)| *start).unwrap_or_default();

        for x in prev_start.. {
            let is_pulled = check_pos(x, y);

            if start.is_none() {
                if is_pulled {
                    start = Some(x);
                } else if x > 10_000 {
                    break;
                }
            } else if !is_pulled {
                end = Some(x - 1);
                break;
            }
        }

        match (start, end) {
            (None, None) => {
                // println!("Line {y}: EMPTY");
                // Looks like an empty line, reset history
                hist.clear();
                continue;
            }
            (None, Some(_)) => unreachable!("Found an end but not start"),
            (Some(_), None) => unreachable!("We should never stop counting once beam is found"),
            (Some(start), Some(end)) => {
                // println!("Line {y}: {start}->{end} ({})", end - start + 1);
                // Push last range in history
                hist.push_front((start, end));
                hist.truncate(BOX_SIZE);
            }
        }

        let max_start = hist.iter().map(|(start, _)| *start).max();
        let min_end = hist.iter().map(|(_, end)| *end).min();

        if hist.len() >= BOX_SIZE {
            let Some(max_start) = max_start else {
                continue;
            };

            let Some(min_end) = min_end else {
                continue;
            };

            if min_end.saturating_sub(max_start) + 1 >= BOX_SIZE {
                return max_start + (y - BOX_SIZE + 1) * 10_000;
            }
        }
    }

    unreachable!()
}

#[allow(clippy::ptr_arg)]
fn check_pos(x: usize, y: usize) -> bool {
    let res = IntcodeComputer::from_file("res/day19.txt").execute(vec![x as isize, y as isize]);

    match &res[..] {
        &[0] => false,
        &[1] => true,
        other => panic!("Expected 0 or 1 but got {:?}", other),
    }
}
