pub struct PermutationIterator<'a, T: 'a> {
    universe: &'a [T],
    size: usize,
    prev: Option<Vec<usize>>,
}

pub fn permutations<T>(universe: &[T], size: usize) -> PermutationIterator<T> {
    PermutationIterator {
        universe,
        size,
        prev: None,
    }
}

fn map<T>(values: &[T], ixs: &[usize]) -> Vec<T>
where
    T: Clone,
{
    ixs.iter().map(|&i| values[i].clone()).collect()
}

impl<'a, T> Iterator for PermutationIterator<'a, T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        let n = self.universe.len();

        if n == 0 {
            return None;
        }

        match self.prev {
            None => {
                let zeroes: Vec<usize> = vec![0;self.size];
                let result = Some(map(self.universe, &zeroes[..]));
                self.prev = Some(zeroes);
                result
            }
            Some(ref mut indexes) => match indexes.iter().position(|&i| i + 1 < n) {
                None => None,
                Some(position) => {
                    for index in indexes.iter_mut().take(position) {
                        *index = 0;
                    }
                    indexes[position] += 1;
                    Some(map(self.universe, &indexes[..]))
                }
            },
        }
    }
}

// use std::slice;

// pub fn permutations<T>(universe: &[T], size: usize) -> Permutations<'_, T> {
//     Permutations {
//         universe,
//         first: true,
//         prev: vec![0; size],
//     }
// }

// pub struct Permutations<'universe, T> {
//     universe: &'universe [T],
//     first: bool,
//     prev: Vec<usize>,
// }

// impl<'universe, T> Permutations<'universe, T> {
//     pub fn next(&mut self) -> Option<Permutation<'universe, '_, T>> {
//         if !self.first {
//             let i = self.prev.iter().position(|&i| i + 1 < self.universe.len())?;
//             self.prev[..i].fill(0);
//             self.prev[i] += 1;
//         }
//         self.first = false;

//         Some(Permutation { universe: self.universe, indices: self.prev.iter() })
//     }
// }

// pub struct Permutation<'universe, 'indices, T> {
//     universe: &'universe [T],
//     indices: slice::Iter<'indices, usize>,
// }

// impl<'universe, T> Iterator for Permutation<'universe, '_, T> {
//     type Item = &'universe T;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(&self.universe[*self.indices.next()?])
//     }
// }