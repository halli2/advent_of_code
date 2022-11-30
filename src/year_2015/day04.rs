use crate::{AdventSolver, Solution};

pub struct DayFour {}

impl AdventSolver for DayFour {
    fn part_one(&self, input: &str) -> Solution {
        let mut i = 1;
        let input = input.trim();
        loop {
            let hash = md5::compute(format!("{input}{i}"));
            let hex = format!("{:x}", hash);
            if hex.starts_with("00000") {
                println!("{hex}");
                return i.into();
            }
            i += 1;
        }
    }

    fn part_two(&self, input: &str) -> Solution {
        let mut i = 1;
        let input = input.trim();
        loop {
            let hash = md5::compute(format!("{input}{i}"));
            let hex = format!("{:x}", hash);
            if hex.starts_with("000000") {
                println!("{hex}");
                return i.into();
            }
            i += 1;
        }
    }
}
