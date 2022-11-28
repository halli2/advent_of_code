use crate::{AdventSolver, Solution};

pub struct DayOne {}

impl AdventSolver for DayOne {
    fn part_one(&self, input: &str) -> Solution {
        let mut res = 0;
        for c in input.chars() {
            match c {
                '(' => res += 1,
                ')' => res -= 1,
                _ => {}
            }
        }
        res.into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let mut res = 0;
        for (index, c) in input.chars().enumerate() {
            match c {
                '(' => res += 1,
                ')' => res -= 1,
                _ => {}
            }
            if res == -1 {
                return (index + 1).into();
            }
        }
        Solution::Unsolved
    }
}
