#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};

pub struct DayOne {}

impl AdventSolver for DayOne {
    fn part_one(&self, input: &str) -> Solution {
        input
            .trim()
            .chars()
            .into_iter()
            .map(|c| if c == '(' { 1 } else { -1 })
            .sum::<i16>()
            .into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let mut res = 0;
        for (index, c) in input.trim().chars().enumerate() {
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
#[cfg(test)]
bench! {2015, 1, DayOne, year_2015}
