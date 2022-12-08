use std::{cmp::Ordering, ops::Range};

use crate::prelude::*;
pub struct DayFour {}

#[allow(dead_code)]
fn section(s: &str) -> Range<u32> {
    s.split_once('-')
        .map(|(left, right)| (left.parse::<u32>().unwrap()..right.parse::<u32>().unwrap()))
        .unwrap()
}
#[allow(dead_code)]
fn sections(s: &str) -> (Range<u32>, Range<u32>) {
    s.split_once(',')
        .map(|(lhs, rhs)| (section(lhs), section(rhs)))
        .unwrap()
}

struct ParserIter<'a, T: ExactSizeIterator<Item = &'a u8>> {
    iter: T,
}

impl<'a, T: ExactSizeIterator<Item = &'a u8>> ParserIter<'a, T> {
    const fn new(iter: T) -> Self {
        Self { iter }
    }
}

// https://github.com/rockisch/rust-aoc-2022/blob/main/src/day04/main.rs
impl<'a, T: ExactSizeIterator<Item = &'a u8>> Iterator for ParserIter<'a, T> {
    type Item = [u16; 4];

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_empty() {
            return None;
        }
        let mut values = [0_u16; 4];
        for value in &mut values {
            let byte_1 = unsafe { *self.iter.next().unwrap_unchecked() };
            let byte_2 = unsafe { *self.iter.next().unwrap_unchecked() };
            // 0-9 0011 0xxx
            // \n  0000 1010
            // ,   0010 1110
            // -   0010 1101
            // &   0001 0000 have to be numeric
            // Don't really care about values, only if it is higher or lower than
            // each other so just make a u16 from the byte values
            if byte_2 & 0b0001_0000 == 0 {
                *value = u16::from_be_bytes([0, byte_1]);
            } else {
                // If 2 digits, iterate again to clear , or - or \n
                *value = u16::from_be_bytes([byte_1, byte_2]);
                unsafe {
                    self.iter.next().unwrap_unchecked();
                }
            }
        }
        Some(values)
    }
}

struct Parser<'a, T: ExactSizeIterator<Item = &'a u8>>(T);

impl<'a, T: ExactSizeIterator<Item = &'a u8>> Parser<'a, T> {
    const fn new(iter: T) -> Self {
        Self(iter)
    }
}

impl<'a, T: ExactSizeIterator<Item = &'a u8>> Iterator for Parser<'a, T> {
    type Item = [u8; 4];

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        let mut values = [0_u8; 4];
        for value in &mut values {
            let byte_1 = unsafe { *self.0.next().unwrap_unchecked() };
            let byte_2 = unsafe { *self.0.next().unwrap_unchecked() };
            let value_1 = byte_1 & 0x0f;
            let value_2 = byte_2 & 0x0f;
            *value = if value_2 > 9 {
                value_1
            } else {
                unsafe { self.0.next().unwrap_unchecked() };
                value_1.wrapping_mul(10).wrapping_add(value_2)
            }
        }
        Some(values)
    }
}

impl AdventSolver for DayFour {
    fn part_one(&self, input: &str) -> Solution {
        ParserIter::new(input.as_bytes().iter())
            .filter(|[left_start, left_end, right_start, right_end]| {
                match left_start.cmp(right_start) {
                    Ordering::Greater => left_end <= right_end,
                    Ordering::Less => left_end >= right_end,
                    Ordering::Equal => true,
                }
            })
            .count()
            .into()
        // input
        //     .split(|c| c == '-' || c == ',' || c == '\n')
        //     .array_chunks::<4>()
        //     .filter(|chunk| {
        //         let end = |chunk: &[&str; 4]| -> (u8, u8) {
        //             (
        //                 chunk[1].parse::<u8>().unwrap(),
        //                 chunk[3].parse::<u8>().unwrap(),
        //             )
        //         };
        //         match chunk[0].len().cmp(&chunk[2].len()) {
        //             Ordering::Less => {
        //                 let (left, right) = end(chunk);
        //                 left >= right
        //             }
        //             Ordering::Greater => {
        //                 let (left, right) = end(chunk);
        //                 left <= right
        //             }
        //             Ordering::Equal => {
        //                 let (left_start, right_start) = {
        //                     (
        //                         chunk[0].parse::<u8>().unwrap(),
        //                         chunk[2].parse::<u8>().unwrap(),
        //                     )
        //                 };

        //                 match left_start.cmp(&right_start) {
        //                     Ordering::Less => {
        //                         let (left, right) = end(chunk);
        //                         left >= right
        //                     }
        //                     Ordering::Equal => true,
        //                     Ordering::Greater => {
        //                         let (left, right) = end(chunk);
        //                         left <= right
        //                     }
        //                 }
        //             }
        //         }
        //     })
        //     .count()
        //     .into()
        // input
        //     .lines()
        //     .into_iter()
        //     .map(sections)
        //     .filter(|(left, right)| match left.start.cmp(&right.start) {
        //         Ordering::Greater => left.end <= right.end,
        //         Ordering::Less => left.end >= right.end,
        //         Ordering::Equal => true,
        //     })
        //     .count()
        //     .into()
    }

    fn part_two(&self, input: &str) -> Solution {
        Parser::new(input.as_bytes().iter())
            .filter(|[left_start, left_end, right_start, right_end]| {
                match left_start.cmp(right_start) {
                    Ordering::Greater => left_start <= right_end,
                    Ordering::Less => left_end >= right_start,
                    Ordering::Equal => true,
                }
            })
            .count()
            .into()
        // input
        //     .lines()
        //     .into_iter()
        //     .map(sections)
        //     .filter(|(left, right)| match left.start.cmp(&right.start) {
        //         Ordering::Greater => left.start <= right.end,
        //         Ordering::Less => left.end >= right.start,
        //         Ordering::Equal => true,
        //     })
        //     .count()
        //     .into()
    }
}

#[cfg(test)]
bench! {2022, 4, DayFour, Solution::Usize(496), Solution::Usize(847)}

#[cfg(test)]
mod tests {
    use super::{AdventSolver, DayFour, Solution};

    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8\n";

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
