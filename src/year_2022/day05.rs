use core::slice;

use arrayvec::ArrayVec;

use crate::prelude::*;
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
        let amount = if *byte_2 == b' ' {
            amount_1
        } else {
            unsafe { self.0.next().unwrap_unchecked() }; // Skip another byte if double digit
            let amount_2 = byte_2 & 0x0f;
            amount_1.wrapping_mul(10).wrapping_add(amount_2)
        };
        // At most 9 stacks so don't need to check for double digits
        // 012345
        // from x
        let from = unsafe { self.0.nth(5).unwrap_unchecked() } & 0x0f;
        // 01234
        //  to x
        let to = unsafe { self.0.nth(4).unwrap_unchecked() } & 0x0f;
        unsafe { self.0.next().unwrap_unchecked() };

        Some(Instruction { amount, from, to })
    }
}

type CharVec = ArrayVec<char, 64>;

impl AdventSolver for DayFive {
    fn part_one(&self, input: &str) -> Solution {
        let input = input.as_bytes();
        let width = unsafe { slice::memchr::memchr(b'9', input).unwrap_unchecked() };
        let (crates, instructions) = input.split_at(width + 3);
        // let (crates, instructions) = input.split_once("\n\n").unwrap();
        let mut stacks: [CharVec; 10] = Default::default();
        // Want stack number first, and upper most crate last so reverse
        let width = unsafe { slice::memchr::memchr(b'\n', crates).unwrap_unchecked() };
        for line in crates.chunks(width + 1).rev().skip(1) {
            // for line in crates.lines().rev().skip(1) {
            // remove first `[`
            // index 1, 5, 9, 13 etc..
            for (index, char) in line.iter().skip(1).step_by(4).enumerate() {
                if *char != b' ' {
                    stacks[index + 1].push(*char as char);
                }
            }
        }
        for instr in InstructionsParser::new(instructions.iter()) {
            for _ in 0..instr.amount {
                let value = unsafe { stacks[instr.from as usize].pop().unwrap_unchecked() };
                stacks[instr.to as usize].push(value);
            }
        }
        stacks
            .iter_mut()
            .filter_map(ArrayVec::pop)
            .collect::<String>()
            .into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let input = input.as_bytes();
        let width = unsafe { slice::memchr::memchr(b'9', input).unwrap_unchecked() };
        let (crates, instructions) = input.split_at(width + 3);
        // let (crates, instructions) = input.split_once("\n\n").unwrap();
        let mut stacks: [CharVec; 10] = Default::default();
        // Want stack number first, and upper most crate last so reverse
        let width = unsafe { slice::memchr::memchr(b'\n', crates).unwrap_unchecked() };
        for line in crates.chunks(width + 1).rev().skip(1) {
            // for line in crates.lines().rev().skip(1) {
            // remove first `[`
            // index 1, 5, 9, 13 etc..
            for (index, char) in line.iter().skip(1).step_by(4).enumerate() {
                if *char != b' ' {
                    stacks[index + 1].push(*char as char);
                }
            }
        }
        // let (crates, instructions) = input.split_once("\n\n").unwrap();
        // let mut stacks: [CharVec; 10] = Default::default();
        // for line in crates.lines().rev().skip(1) {
        //     // skip first `[`
        //     // index 1, 5, 9, 13 etc..
        //     for (index, char) in line.chars().skip(1).step_by(4).enumerate() {
        //         if char != ' ' {
        //             stacks[index + 1].push(char);
        //         }
        //     }
        // }
        for instr in InstructionsParser::new(instructions.iter()) {
            unsafe {
                // let [from, to] = [instr.from, instr.to]
                //     .map(|i| &mut *std::ptr::addr_of_mut!(stacks[i as usize]));
                let from = &mut *std::ptr::addr_of_mut!(stacks[instr.from as usize]);
                let to = &mut *std::ptr::addr_of_mut!(stacks[instr.to as usize]);
                let last = from.len();
                let values = from.drain((last - instr.amount as usize)..);
                to.extend(values);
            }
        }
        stacks
            .iter_mut()
            .filter_map(ArrayVec::pop)
            .collect::<String>()
            .into()
    }
}

#[cfg(test)]
bench! {2022, 5, DayFive, Solution::String("JCMHLVGMG".to_owned()), Solution::String("LVMRWSSPZ".to_owned())}

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
