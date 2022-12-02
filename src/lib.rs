#![feature(test)]
extern crate test;

mod error;
pub mod gpu;
pub mod parser;

#[cfg(test)]
/// `bench {2022, 1, DayOne, year_2022, (answer_1), (answer_2)}`
macro_rules! bench {
    ($year:literal, $day:tt, $struct:ident, $year_mod:ident) => {
        mod tests {
            use std::fs;

            use test::{black_box, Bencher};

            use crate::{$year_mod::$struct, AdventSolver};

            #[bench]
            fn part1(b: &mut Bencher) {
                let content =
                    fs::read_to_string(format!("./data/{}/day{:0>2}.txt", $year, $day)).unwrap();
                let day = $struct {};
                b.iter(|| {
                    black_box(day.part_one(black_box(&content)));
                })
            }

            #[bench]
            fn part2(b: &mut Bencher) {
                let content =
                    fs::read_to_string(format!("./data/{}/day{:0>2}.txt", $year, $day)).unwrap();
                let day = $struct {};
                b.iter(|| {
                    black_box(day.part_two(black_box(&content)));
                })
            }
        }
    };
    ($year:literal, $day:tt, $struct:ident, $year_mod:ident, $answer_1:expr, $answer_2:expr) => {
        mod tests {
            use std::fs;

            use test::{black_box, Bencher};

            use crate::{$year_mod::$struct, AdventSolver, Solution};

            #[bench]
            fn part1(b: &mut Bencher) {
                let content =
                    fs::read_to_string(format!("./data/{}/day{:0>2}.txt", $year, $day)).unwrap();
                let day = $struct {};
                b.iter(|| {
                    black_box(day.part_one(black_box(&content)));
                });
                let answer = day.part_one(&content);
                assert_eq!($answer_1, answer);
            }
            #[bench]
            fn part2(b: &mut Bencher) {
                let content =
                    fs::read_to_string(format!("./data/{}/day{:0>2}.txt", $year, $day)).unwrap();
                let day = $struct {};
                b.iter(|| {
                    black_box(day.part_two(black_box(&content)));
                });
                let answer = day.part_two(&content);
                assert_eq!($answer_2, answer);
            }
        }
    };
}

macro_rules! day {
    ($($index:literal::$day:ident::$struct:ident), +) => {
        $(pub mod $day;
        pub use $day::$struct;)*


        use crate::AdventSolver;
        pub fn get_day(day: u32) -> Box<dyn AdventSolver> {
            match day {
                $($index => Box::new($struct {}),)*
                _ => unimplemented!("Day {day} is unimplemented")
            }
        }
    }
}

pub mod year_2015 {
    //! Warm up for 2022
    day! {
        1::day01::DayOne,
        2::day02::DayTwo,
        3::day03::DayThree,
        4::day04::DayFour
    }
}

pub mod year_2022 {
    day! {
        0::day00::DayZero,
        1::day01::DayOne,
        2::day02::DayTwo,
        3::day03::DayThree
    }
}

pub trait AdventSolver {
    fn part_one(&self, input: &str) -> Solution;
    fn part_two(&self, input: &str) -> Solution;
    fn visualize(&self, _input: &str) {}
}

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    I32(i32),
    U8(u8),
    U16(u16),
    U32(u32),
    VecU32(Vec<u32>),
    Usize(usize),
    String(String),
    Unsolved,
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::I32(v) => v.fmt(f),
            Solution::U8(v) => v.fmt(f),
            Solution::U16(v) => v.fmt(f),
            Solution::U32(v) => v.fmt(f),
            Solution::VecU32(v) => {
                for v in v {
                    write!(f, "{v} ")?;
                }
                Ok(())
            }
            Solution::Usize(v) => v.fmt(f),
            Solution::String(v) => v.fmt(f),
            Solution::Unsolved => write!(f, "Something wrong happened, problem not solved"),
        }
    }
}

macro_rules! from {
    ($from:ident, $to:ident) => {
        impl From<$from> for Solution {
            fn from(value: $from) -> Self {
                Self::$to(value)
            }
        }
    };
}
from! {usize, Usize}
from! {i32, I32}
from! {u8, U8}
from! {u16, U16}
from! {u32, U32}
from! {String, String}

impl From<Vec<u32>> for Solution {
    fn from(v: Vec<u32>) -> Self {
        Self::VecU32(v)
    }
}
