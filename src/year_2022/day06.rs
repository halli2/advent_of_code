#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};
pub struct DaySix {}

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
#[inline(always)]
fn xor_dups<const N: usize>(input: &[u8]) -> Solution {
    let f = |b: u8| -> u32 { 1_u32 << (b % b'a') };
    let mut masks = input[..N].iter().fold(0, |a, &b| a ^ f(b));
    for (index, window) in input.windows(N + 1).enumerate() {
        if masks.count_ones() == N as u32 {
            return (index + N).into();
        }
        masks ^= f(window[N]) ^ f(window[0]);
    }
    Solution::Unsolved
}

impl AdventSolver for DaySix {
    fn part_one(&self, input: &str) -> Solution {
        const N: usize = 4;
        let input = input.as_bytes();
        for (index, window) in input.windows(N).enumerate() {
            if !dups(window) {
                return (index + 4).into();
            }
        }
        Solution::Unsolved
        // xor_dups::<N>(input)
    }

    fn part_two(&self, input: &str) -> Solution {
        const N: usize = 14;
        let input = input.as_bytes();
        // for (index, window) in input.windows(N).enumerate() {
        //     if !dups(window) {
        //         return (index + 14).into();
        //     }
        // }
        // Solution::Unsolved
        xor_dups::<N>(input)
    }
}

#[cfg(test)]
bench! {2022, 6, DaySix, year_2022, Solution::Usize(1965), Solution::Usize(2773)}

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
