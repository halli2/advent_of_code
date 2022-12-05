use std::cell::UnsafeCell;

use arrayvec::ArrayVec;

#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};
pub struct DayFive {}

struct InstructionsParser<'a, T: ExactSizeIterator<Item = &'a u8>>(T);

impl<'a, T: ExactSizeIterator<Item = &'a u8>> InstructionsParser<'a, T> {
    const fn new(iter: T) -> Self {
        Self(iter)
    }
}

#[derive(Debug)]
struct Instruction {
    pub amount: u8,
    pub from: u8,
    pub to: u8,
}

impl<'a, T: ExactSizeIterator<Item = &'a u8>> Iterator for InstructionsParser<'a, T> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        // 012345
        // move x
        let amount_1 = unsafe { self.0.nth(5).unwrap_unchecked() } & 0x0f;
        let byte_2 = unsafe { self.0.next().unwrap_unchecked() };
        // If space
        let amount = if *byte_2 == b' ' {
            amount_1
        } else {
            unsafe { self.0.next().unwrap_unchecked() }; // iter to next value
            let amount_2 = byte_2 & 0x0f;
            amount_1.wrapping_mul(10).wrapping_add(amount_2)
        };
        // At most 9 crates(?) so don't need to check for double digits
        // `012345
        //  from x`
        // Index 1 - 9
        let from = unsafe { self.0.nth(5).unwrap_unchecked() } & 0x0f;
        // 01234
        //  to x
        let to = unsafe { self.0.nth(4).unwrap_unchecked() } & 0x0f;
        unsafe { self.0.next().unwrap_unchecked() };

        Some(Instruction { amount, from, to })
    }
}

impl AdventSolver for DayFive {
    fn part_one(&self, input: &str) -> Solution {
        let (crates, instructions) = input.split_once("\n\n").unwrap();
        // stacks of crates
        let mut stacks: [ArrayVec<char, 50>; 10] = Default::default();
        // Want stack number first, and upper most crate last so reverse
        for line in crates.lines().rev().skip(1) {
            // remove first `[`
            // index 1, 5, 9, 13 etc..

            for (index, char) in line.chars().skip(1).step_by(4).enumerate() {
                if char != ' ' {
                    stacks[index + 1].push(char);
                }
            }
        }

        for instr in InstructionsParser::new(instructions.as_bytes().iter()) {
            for _ in 0..instr.amount {
                let value = unsafe { stacks[instr.from as usize].pop().unwrap_unchecked() };
                stacks[instr.to as usize].push(value);
            }
        }
        let mut result = String::with_capacity(9);
        for stack in &mut stacks {
            if let Some(v) = stack.pop() {
                result.push(v);
            }
        }
        result.into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let (crates, instructions) = input.split_once("\n\n").unwrap();
        let stacks: [UnsafeCell<ArrayVec<char, 60>>; 10] = Default::default();
        for line in crates.lines().rev().skip(1) {
            // remove first `[`
            // index 1, 5, 9, 13 etc..
            for (index, char) in line.chars().skip(1).step_by(4).enumerate() {
                if char != ' ' {
                    unsafe {
                        let inner: &mut ArrayVec<char, 60> = &mut *stacks[index + 1].get();
                        inner.push(char);
                    }
                }
            }
        }
        for instr in InstructionsParser::new(instructions.as_bytes().iter()) {
            unsafe {
                let inner_from: &mut ArrayVec<char, 60> = &mut *stacks[instr.from as usize].get();
                let inner_to: &mut ArrayVec<char, 60> = &mut *stacks[instr.to as usize].get();
                let last = inner_from.len();
                let values = inner_from.drain((last - instr.amount as usize)..);

                inner_to.extend(values);
            }
        }

        let mut result = String::with_capacity(9);
        for vec in stacks {
            let stack: &mut ArrayVec<char, 60> = unsafe { &mut *vec.get() };
            if let Some(v) = stack.pop() {
                result.push(v);
            }
        }
        result.into()
        // let (crates, instructions) = input.split_once("\n\n").unwrap();
        // // stacks of crates
        // let mut stacks: [ArrayVec<char, 50>; 10] = Default::default();
        // // let mut stacks: Vec<Vec<char>> = (0..10)
        // //     .into_iter()
        // //     .map(|_| Vec::with_capacity(50))
        // //     .collect();
        // // Want stack number first, and upper most crate last so reverse
        // for line in crates.lines().rev().skip(1) {
        //     // remove first `[`
        //     // index 1, 5, 9, 13 etc..
        //     for (index, char) in line.chars().skip(1).step_by(4).enumerate() {
        //         if char != ' ' {
        //             stacks[index + 1].push(char);
        //         }
        //     }
        // }
        // let mut intermediate = ArrayVec::<_, 50>::new();
        // for instr in InstructionsParser::new(instructions.as_bytes().iter()) {
        //     (0..instr.amount)
        //         .into_iter()
        //         .map(|_| unsafe { stacks[instr.from as usize].pop().unwrap_unchecked() })
        //         .collect_into(&mut intermediate);
        //     intermediate.reverse();
        //     stacks[instr.to as usize].extend(&mut intermediate.drain(..));
        // }

        // let mut result = String::with_capacity(9);
        // for stack in &mut stacks {
        //     if let Some(v) = stack.pop() {
        //         result.push(v);
        //     }
        // }
        // result.into()
    }
}

#[cfg(test)]
bench! {2022, 5, DayFive, year_2022, Solution::String("JCMHLVGMG".to_owned()), Solution::String("LVMRWSSPZ".to_owned())}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"\
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn part1() {
        let result: Solution = "CMZ".into();
        let day = DayFive {};
        assert_eq!(day.part_one(INPUT), result);
    }
    #[test]
    fn part2() {
        let result: Solution = "MCD".into();
        let day = DayFive {};
        assert_eq!(day.part_two(INPUT), result);
    }
}
