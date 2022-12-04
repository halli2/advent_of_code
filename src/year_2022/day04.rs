use std::{cmp::Ordering, ops::Range};

#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};
pub struct DayFour {}

fn section(s: &str) -> Range<u32> {
    s.split_once('-')
        .map(|(left, right)| (left.parse::<u32>().unwrap()..right.parse::<u32>().unwrap()))
        .unwrap()
}
fn sections(s: &str) -> (Range<u32>, Range<u32>) {
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
            .filter(|(left, right)| match left.start.cmp(&right.start) {
                Ordering::Greater => left.end <= right.end,
                Ordering::Less => left.end >= right.end,
                Ordering::Equal => true,
            })
            .count()
            .into()
    }

    fn part_two(&self, input: &str) -> Solution {
        input
            .lines()
            .into_iter()
            .map(sections)
            .filter(|(left, right)| match left.start.cmp(&right.start) {
                Ordering::Greater => left.start <= right.end,
                Ordering::Less => left.end >= right.start,
                Ordering::Equal => true,
            })
            .count()
            .into()
    }
}

#[cfg(test)]
bench! {2022, 4, DayFour, year_2022, Solution::Usize(496), Solution::Usize(847)}

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
        let result = Solution::Usize(2);
        let day = DayFour {};
        assert_eq!(day.part_one(INPUT), result);
    }

    #[test]
    fn part2() {
        let result = Solution::Usize(4);
        let day = DayFour {};
        assert_eq!(day.part_two(INPUT), result);
    }
}
