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

impl AdventSolver for DaySix {
    fn part_one(&self, input: &str) -> Solution {
        for (index, window) in input.as_bytes().windows(4).enumerate() {
            if !dups(window) {
                return (index + 4).into();
            }
        }
        Solution::Unsolved
    }

    fn part_two(&self, input: &str) -> Solution {
        for (index, window) in input.as_bytes().windows(14).enumerate() {
            if !dups(window) {
                return (index + 14).into();
            }
        }
        Solution::Unsolved
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
