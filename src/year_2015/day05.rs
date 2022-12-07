#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};
pub struct DayFive {}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

impl AdventSolver for DayFive {
    fn part_one(&self, input: &str) -> Solution {
        let mut good = 0;
        for line in input.lines() {
            let mut vovels = 0;
            for character in line.chars() {
                if VOWELS.contains(&character) {
                    vovels += 1;
                    if vovels == 3 {
                        good += 1;
                        continue;
                    }
                }
            }
        }
        good.into()
    }

    fn part_two(&self, _input: &str) -> Solution {
        todo!()
    }
}

#[cfg(test)]
bench! {2015, 5, DayFive, year_2015}
