use std::cmp::Ordering;

#[cfg(test)]
use crate::bench;

use crate::{AdventSolver, Solution};

pub struct DayTwo {}
#[inline(always)]
fn get(input: &[u8], i: usize) -> u8 {
    unsafe { *input.as_ptr().add(i) }
}

impl AdventSolver for DayTwo {
    fn part_one(&self, input: &str) -> Solution {
        // X - Rock
        // Y - Paper
        // Z - Scissor
        let input = input.as_bytes();
        let mut i = 0;
        let mut total = 0;
        let len = input.len();
        while i < len {
            let op = get(input, i) as i32 - 64;
            let strat = get(input, i + 2) as i32 - 87;
            total += (strat - op + 4) % 3 * 3 + strat;
            i += 4;
        }
        total.into()
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
    }

    fn part_two(&self, input: &str) -> Solution {
        // X - Lose
        // Y - Draw
        // Z - Win

        let input = input.as_bytes();
        let mut i = 0;
        let mut total = 0;
        let len = input.len();
        while i < len {
            let op = get(input, i) as u32 - 64;
            let strat = (get(input, i + 2) as u32 - 88) * 3;
            total += (strat / 3 + op + 1) % 3 + 1 + strat;
            i += 4;
        }
        total.into()
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
