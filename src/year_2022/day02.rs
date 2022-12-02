use std::cmp::Ordering;

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
            .map(|l| {
                (
                    // map to 1-3
                    l.chars().next().unwrap() as i32 - 64,
                    l.chars().nth(2).unwrap() as i32 - 87,
                )
            })
            .map(|(opp, strat)| (strat - opp + 4) % 3 * 3 + strat)
            .sum::<i32>()
            .into()
    }
    //     input
    //         .lines()
    //         .into_iter()
    //         .fold(0u32, |score, line| match line {
    //             "A X" => score + 4,
    //             "A Y" => score + 8,
    //             "A Z" => score + 3,
    //             "B X" => score + 1,
    //             "B Y" => score + 5,
    //             "B Z" => score + 9,
    //             "C X" => score + 7,
    //             "C Y" => score + 2,
    //             "C Z" => score + 6,
    //             _ => unreachable!(),
    //         })
    //         .into()
    // }

    fn part_two(&self, input: &str) -> Solution {
        // X - Lose
        // Y - Draw
        // Z - Win
        input
            .lines()
            .into_iter()
            .map(|l| {
                (
                    // map to 1-3
                    l.chars().next().unwrap() as i32 - 64,
                    // map to 0, 3, 6
                    (l.chars().nth(2).unwrap() as i32 - 88) * 3,
                )
            })
            .map(|(opp, strat)| (strat / 3 + opp + 1) % 3 + 1 + strat)
            .sum::<i32>()
            .into()
        // input
        //     .lines()
        //     .into_iter()
        //     .fold(0u32, |score, line| match line {
        //         "A X" => score + 3,
        //         "B X" => score + 1,
        //         "C X" => score + 2,
        //         "A Y" => score + 4,
        //         "B Y" => score + 5,
        //         "C Y" => score + 6,
        //         "A Z" => score + 8,
        //         "B Z" => score + 9,
        //         "C Z" => score + 7,
        //         _ => unreachable!(),
        //     })
        //     .into()
    }
}

#[cfg(test)]
bench! {2022, 2, DayTwo, year_2022}
