use core::slice;
use std::mem;

use crate::{AdventSolver, Solution};

pub struct DayTwo {}

fn cast<T>(data: &[u8]) -> &[T] {
    unsafe {
        slice::from_raw_parts(
            (data as *const [u8]).cast::<T>(),
            data.len() / mem::size_of::<T>(),
        )
    }
}

const AX: u32 = 173547585;
const AY: u32 = 173613121;
const AZ: u32 = 173678657;
const BX: u32 = 173547586;
const BY: u32 = 173613122;
const BZ: u32 = 173678658;
const CX: u32 = 173547587;
const CY: u32 = 173613123;
const CZ: u32 = 173678659;

const ANSWERS_1: [u32; 20] = {
    let mut a = [0_u32; 20];
    a[(AX.wrapping_mul(AX) >> 27) as usize] = 4; // Tie + Rock = 4
    a[(AY.wrapping_mul(AX) >> 27) as usize] = 8; // Win + Paper = 8
    a[(AZ.wrapping_mul(AX) >> 27) as usize] = 3; // etc.
    a[(BX.wrapping_mul(AX) >> 27) as usize] = 1;
    a[(BY.wrapping_mul(AX) >> 27) as usize] = 5;
    a[(BZ.wrapping_mul(AX) >> 27) as usize] = 9;
    a[(CX.wrapping_mul(AX) >> 27) as usize] = 7;
    a[(CY.wrapping_mul(AX) >> 27) as usize] = 2;
    a[(CZ.wrapping_mul(AX) >> 27) as usize] = 6;
    a
};

const ANSWERS_2: [u32; 20] = {
    let mut a = [0_u32; 20];
    a[(AX.wrapping_mul(AX) >> 27) as usize] = 3;
    a[(AY.wrapping_mul(AX) >> 27) as usize] = 4;
    a[(AZ.wrapping_mul(AX) >> 27) as usize] = 8;
    a[(BX.wrapping_mul(AX) >> 27) as usize] = 1;
    a[(BY.wrapping_mul(AX) >> 27) as usize] = 5;
    a[(BZ.wrapping_mul(AX) >> 27) as usize] = 9;
    a[(CX.wrapping_mul(AX) >> 27) as usize] = 2;
    a[(CY.wrapping_mul(AX) >> 27) as usize] = 6;
    a[(CZ.wrapping_mul(AX) >> 27) as usize] = 7;
    a
};
impl AdventSolver for DayTwo {
    fn part_one(&self, input: &str) -> Solution {
        let input = cast::<u32>(input.as_bytes());
        input
            .iter()
            .map(|l| ANSWERS_1[(l.wrapping_mul(AX) >> 27) as usize])
            .sum::<u32>()
            .into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let input = cast::<u32>(input.as_bytes());
        input
            .iter()
            .map(|l| ANSWERS_2[(l.wrapping_mul(AX) >> 27) as usize])
            .sum::<u32>()
            .into()
    }
}

#[cfg(test)]
bench! {2022, 2, DayTwo, year_2022, Solution::U32(11906), Solution::U32(11186)}

// impl AdventSolver for DayTwo {
//     fn part_one(&self, input: &str) -> Solution {
//         // X - Rock
//         // Y - Paper
//         // Z - Scissor
//         input
//             .lines()
//             .into_iter()
//             .fold(0u32, |score, line| match line {
//                 "A X" => score + 4,
//                 "A Y" => score + 8,
//                 "A Z" => score + 3,
//                 "B X" => score + 1,
//                 "B Y" => score + 5,
//                 "B Z" => score + 9,
//                 "C X" => score + 7,
//                 "C Y" => score + 2,
//                 "C Z" => score + 6,
//                 _ => unreachable!(),
//             })
//             .into()
//         }
//     }
//     fn part_two(&self, input: &str) -> Solution {
//         // X - Lose
//         // Y - Draw
//         // Z - Win
//         input
//             .lines()
//             .into_iter()
//             .fold(0u32, |score, line| match line {
//                 "A X" => score + 3,
//                 "B X" => score + 1,
//                 "C X" => score + 2,
//                 "A Y" => score + 4,
//                 "B Y" => score + 5,
//                 "C Y" => score + 6,
//                 "A Z" => score + 8,
//                 "B Z" => score + 9,
//                 "C Z" => score + 7,
//                 _ => unreachable!(),
//             })
//             .into()
//     }
// }
