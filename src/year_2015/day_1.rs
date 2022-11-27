use crate::AdventSolver;

pub struct DayOne {}

impl AdventSolver for DayOne {
    fn part_one(&self, input: &str) -> String {
        let mut res = 0;
        for c in input.chars() {
            match c {
                '(' => res += 1,
                ')' => res -= 1,
                _ => {}
            }
        }
        res.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut res = 0;
        for (index, c) in input.chars().enumerate() {
            match c {
                '(' => res += 1,
                ')' => res -= 1,
                _ => {}
            }
            if res == -1 {
                return (index + 1).to_string();
            }
        }
        "Does not enter basement".to_string()
    }
}
