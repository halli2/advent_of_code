#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};
pub struct DayFour {}

fn section(s: &str) -> (u32, u32) {
    s.split_once('-')
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .unwrap()
}

/// Section1 (u32..u32), section2 (u32..u32)
fn sections(s: &str) -> ((u32, u32), (u32, u32)) {
    s.split_once(',')
        .map(|(lhs, rhs)| (section(lhs), section(rhs)))
        .unwrap()
}

impl AdventSolver for DayFour {
    fn part_one(&self, input: &str) -> Solution {
        input
            .lines()
            .into_iter()
            .map(sections)
            .map(|((left_1, left_2), (right_1, right_2))| {
                match (left_1, right_1) {
                    (left, right) if left > right => {
                        if left_2 <= right_2 {
                            return 1;
                        }
                    }
                    (left, right) if left < right => {
                        if left_2 >= right_2 {
                            return 1;
                        }
                    }
                    (left, right) if left == right => {
                        return 1;
                    }
                    _ => unreachable!(),
                }
                0
            })
            .sum::<u32>()
            .into()
    }

    fn part_two(&self, input: &str) -> Solution {
        input
            .lines()
            .into_iter()
            .map(sections)
            .map(|((left_1, left_2), (right_1, right_2))| {
                match (left_1, right_1) {
                    (left, right) if left > right => {
                        if left <= right_2 {
                            return 1;
                        }
                    }
                    (left, right) if left < right => {
                        if left_2 >= right {
                            return 1;
                        }
                    }
                    (left, right) if left == right => {
                        return 1;
                    }
                    _ => unreachable!(),
                }
                0
            })
            .sum::<u32>()
            .into()
    }
}

#[cfg(test)]
bench! {2022, 4, DayFour, year_2022, Solution::U32(496), Solution::U32(847)}

#[cfg(test)]
mod tests {
    use super::{AdventSolver, DayFour, Solution};

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1() {
        let result = Solution::U32(2);
        let day = DayFour {};
        assert_eq!(day.part_one(INPUT), result);
    }

    #[test]
    fn part2() {
        let result = Solution::U32(4);
        let day = DayFour {};
        assert_eq!(day.part_two(INPUT), result);
    }
}
