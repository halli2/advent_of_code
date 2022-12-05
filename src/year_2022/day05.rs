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
        let amount = if *byte_2 == 0b0010_0000 {
            amount_1
        } else {
            unsafe { self.0.next().unwrap_unchecked() }; // iter to next value
            let amount_2 = byte_2 & 0x0f;
            amount_1.wrapping_mul(10).wrapping_add(amount_2)
        };
        // At most 9 crates(?) so don't need to check for double digits
        // 012345
        // from x
        let from = (unsafe { self.0.nth(5).unwrap_unchecked() } & 0x0f) - 1;
        // 01234
        //  to x
        let to = (unsafe { self.0.nth(4).unwrap_unchecked() } & 0x0f) - 1;
        unsafe { self.0.next().unwrap_unchecked() };

        Some(Instruction { amount, from, to })
    }
}

impl AdventSolver for DayFive {
    fn part_one(&self, input: &str) -> Solution {
        let (crates, instructions) = input.split_once("\n\n").unwrap();
        // stacks of crates
        let mut stacks: Vec<Vec<char>> = (0..9).into_iter().map(|_| Vec::new()).collect();
        // Want stack number first, and upper most crate last so reverse
        let mut lines = crates.lines().rev();
        lines.next();
        for line in lines {
            // remove first `[`
            let chars = line.chars().skip(1);
            // index 1, 5, 9, 13 etc..
            for (index, char) in chars.step_by(4).enumerate() {
                if char != ' ' {
                    stacks[index].push(char);
                }
            }
        }
        for instr in InstructionsParser::new(instructions.as_bytes().iter()) {
            for _ in 0..instr.amount {
                let value = stacks[instr.from as usize].pop().unwrap();
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
        // stacks of crates
        let mut stacks: Vec<Vec<char>> = (0..9).into_iter().map(|_| Vec::new()).collect();
        // Want stack number first, and upper most crate last so reverse
        let mut lines = crates.lines().rev();
        lines.next();
        for line in lines {
            // remove first `[`
            let chars = line.chars().skip(1);
            // index 1, 5, 9, 13 etc..
            for (index, char) in chars.step_by(4).enumerate() {
                if char != ' ' {
                    stacks[index].push(char);
                }
            }
        }
        for instr in InstructionsParser::new(instructions.as_bytes().iter()) {
            let mut values: Vec<_> = (0..instr.amount)
                .into_iter()
                .map(|_| stacks[instr.from as usize].pop().unwrap())
                .collect();
            values.reverse();
            stacks[instr.to as usize].append(&mut values);
        }

        let mut result = String::with_capacity(9);
        for stack in &mut stacks {
            if let Some(v) = stack.pop() {
                result.push(v);
            }
        }
        result.into()
    }
}

#[cfg(test)]
bench! {2022, 5, DayFive, year_2022}

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
