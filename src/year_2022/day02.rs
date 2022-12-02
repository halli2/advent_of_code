#[cfg(test)]
use crate::bench;

use crate::{AdventSolver, Solution};

pub struct DayTwo {}

impl AdventSolver for DayTwo {
    fn part_one(&self, input: &str) -> Solution {
        // X - Rock
        // Y - Paper
        // Z - Scissor
        input
            .lines()
            .into_iter()
            .fold(0u32, |score, line| {
                let mut play = line.split_whitespace();
                match (play.next(), play.next()) {
                    (Some("A"), Some("X")) => score + 4,
                    (Some("A"), Some("Y")) => score + 8,
                    (Some("A"), Some("Z")) => score + 3,
                    (Some("B"), Some("X")) => score + 1,
                    (Some("B"), Some("Y")) => score + 5,
                    (Some("B"), Some("Z")) => score + 9,
                    (Some("C"), Some("X")) => score + 7,
                    (Some("C"), Some("Y")) => score + 2,
                    (Some("C"), Some("Z")) => score + 6,
                    _ => unreachable!(),
                }
            })
            .into()
    }

    fn part_two(&self, input: &str) -> Solution {
        // X - Lose
        // Y - Draw
        // Z - Win
        input
            .lines()
            .into_iter()
            .fold(0u32, |score, line| {
                let mut play = line.split_whitespace();
                match (play.next(), play.next()) {
                    (Some("A"), Some("X")) => score + 3,
                    (Some("B"), Some("X")) => score + 1,
                    (Some("C"), Some("X")) => score + 2,
                    (Some("A"), Some("Y")) => score + 4,
                    (Some("B"), Some("Y")) => score + 5,
                    (Some("C"), Some("Y")) => score + 6,
                    (Some("A"), Some("Z")) => score + 8,
                    (Some("B"), Some("Z")) => score + 9,
                    (Some("C"), Some("Z")) => score + 7,
                    _ => unreachable!(),
                }
            })
            .into()
    }
}

#[cfg(test)]
bench! {2022, 2, DayTwo, year_2022}
