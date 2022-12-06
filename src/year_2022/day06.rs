use std::mem;

#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};
pub struct DaySix {}

impl AdventSolver for DaySix {
    fn part_one(&self, input: &str) -> Solution {
        let [mut one, mut two, mut three, mut four] = [' '; 4];
        for (ind, character) in input.chars().enumerate() {
            mem::swap(&mut four, &mut three);
            mem::swap(&mut three, &mut two);
            mem::swap(&mut two, &mut one);
            one = character;
            if one != two
                && one != three
                && one != four
                && four != ' ' // First
                && two != three
                && two != four
                && three != four
            {
                return (ind + 1).into();
            }
        }
        Solution::Unsolved
    }

    fn part_two(&self, input: &str) -> Solution {
        let mut queue = [' '; 14];
        for (ind, character) in input.chars().enumerate() {
            for i in (1..14).rev() {
                let value = queue[i - 1];
                queue[i] = value;
            }
            queue[0] = character;
            let mut dup = false;
            for (ind, i) in queue.iter().enumerate() {
                for j in &queue[(ind + 1)..] {
                    if i == j {
                        dup = true;
                    }
                }
            }
            if !dup && queue[13] != ' ' {
                return (ind + 1).into();
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
