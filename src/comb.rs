use std::fmt::Debug;

pub fn permutations<T>(from: &[T]) -> Vec<Vec<T>>
where
    T: Clone + Debug + Sized,
{
    if from.is_empty() {
        return vec![vec![]];
    }

    let mut ret = vec![];

    for (i, e) in from.iter().enumerate() {
        let (left, right) = from.split_at(i);

        for mut combo in permutations(&[left, &right[1..]].concat()).into_iter() {
            combo.push(e.clone());
            ret.push(combo);
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    use std::{collections::HashSet, iter::FromIterator};

    #[test]
    fn permutations_test() {
        let permutations: HashSet<Vec<usize>> = HashSet::from_iter(permutations(&[1, 2, 3]));
        let expected: HashSet<Vec<usize>> = HashSet::from_iter([
            vec![3, 2, 1],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![1, 2, 3],
        ]);

        assert_eq!(permutations, expected);
    }
}
