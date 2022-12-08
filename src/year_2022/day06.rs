use crate::prelude::*;
pub struct DaySix {}

/// General way
#[allow(dead_code)]
#[inline(always)]
fn dups(arr: &[u8]) -> bool {
    for (ind, i) in arr.iter().enumerate() {
        for j in &arr[(ind + 1)..] {
            if i == j {
                return true;
            }
        }
    }
    false
}

/// Optimized way, only works when value types at most 32 and N at most 32
// #[allow(dead_code)]
// #[inline(always)]
// fn xor_dups<const N: usize>(input: &[u8]) -> Solution {
//     let mut mask: u32 = input[..N].iter().fold(0, |a, &byte| a ^ (1 << (byte % 32)));
//     for (index, window) in input.windows(N + 1).enumerate() {
//         if mask.count_ones() == N as u32 {
//             return (index + N).into();
//         }
//         mask ^= (1 << (window[N] % 32)) ^ (1 << (window[0] % 32));
//     }
//     Solution::Unsolved
// }
#[allow(dead_code)]
#[inline(always)]
fn xor_dups<const N: usize>(input: &[u8]) -> usize {
    let mut mask: u32 = input[..N].iter().fold(0, |a, &byte| a ^ (1 << (byte % 32)));
    input
        .windows(N + 1)
        .position(|window| {
            if mask.count_ones() == N as u32 {
                true
            } else {
                mask ^= (1 << (window[N] % 32)) ^ (1 << (window[0] % 32));
                false
            }
        })
        .unwrap()
        + N
}

impl AdventSolver for DaySix {
    fn part_one(&self, input: &str) -> Solution {
        const N: usize = 4;
        let input = input.as_bytes();
        (input.windows(N).position(|window| !dups(window)).unwrap() + N).into()
    }

    fn part_two(&self, input: &str) -> Solution {
        const N: usize = 14;
        let input = input.as_bytes();
        xor_dups::<N>(input).into()
    }
}

#[cfg(test)]
bench! {2022, 6, DaySix, Solution::Usize(1965), Solution::Usize(2773)}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "\
mjqjpqmgbljsphdztnvjfqwrcgsmlb
";

    #[test]
    fn part1() {
        let result: Solution = 7_usize.into();
        let day = DaySix {};
        assert_eq!(day.part_one(INPUT), result);
    }
    #[test]
    fn part2() {
        let result: Solution = 19_usize.into();
        let day = DaySix {};
        assert_eq!(day.part_two(INPUT), result);
    }
}
