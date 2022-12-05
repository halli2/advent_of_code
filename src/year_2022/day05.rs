#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};
pub struct DayFive {}

// struct FastParser<'a, T: ExactSizeIterator<Item = &'a u8>>(T);

impl AdventSolver for DayFive {
    fn part_one(&self, _input: &str) -> Solution {
        todo!()
    }

    fn part_two(&self, _input: &str) -> Solution {
        todo!()
    }
}

#[cfg(test)]
bench! {2022, 5, DayFive, year_2022}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "\
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2\n";

    #[test]
    fn part1() {
        let result: Solution = "CMZ".into();
        let solver = DayFive {};
    }
}
