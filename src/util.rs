use std::collections::{BTreeSet, HashMap, HashSet};

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

pub fn a_star<S, N, H>(start: S, end: S, nexts: N, heur: H) -> Option<(Vec<S>, usize)>
where
    S: Clone + std::hash::Hash + PartialEq + Eq + PartialOrd + Ord,
    N: Fn(&S) -> HashSet<(S, usize)>,
    H: Fn(&S) -> usize,
{
    let mut prevs: HashMap<S, S> = HashMap::new();
    let mut dists: HashMap<S, usize> = HashMap::from([(start.clone(), 0)]);
    let mut to_visit: BTreeSet<(usize, S)> = BTreeSet::from([(0, start.clone())]);

    while let Some((_, mut curr)) = to_visit.pop_first() {
        if curr == end {
            let mut path = vec![curr.clone()];
            while curr != start {
                curr = prevs.get(&curr).unwrap().clone();
                path.push(curr.clone());
            }
            path.reverse();
            return Some((path, *dists.get(&end).unwrap()));
        }

        for (next, cost) in nexts(&curr) {
            let dist = cost + *dists.get(&curr).unwrap();

            if *dists.get(&next).unwrap_or(&usize::MAX) > dist {
                dists.insert(next.clone(), dist);
                prevs.insert(next.clone(), curr.clone());
                to_visit.insert((dist + heur(&next), next));
            }
        }
    }

    None
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
