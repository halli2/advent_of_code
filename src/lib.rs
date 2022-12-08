#![feature(slice_internals)]
#![feature(array_windows)]
#![feature(exact_size_is_empty)]
#![feature(iter_collect_into)]
#![feature(iter_array_chunks)]
#![feature(test)]
#![feature(slice_swap_unchecked)]
#![feature(type_alias_impl_trait)]
extern crate test;

pub mod array;
mod error;
pub mod gpu;
pub mod parser;
pub mod utils;

pub mod prelude {
    pub use std::ops::{Index, IndexMut};

    pub use crate::{array::*, utils::*, AdventSolver, Solution};
    #[cfg(test)]
    pub use crate::{bench, test};
}

pub trait AdventSolver {
    fn part_one(&self, input: &str) -> Solution;
    fn part_two(&self, input: &str) -> Solution;
    fn visualize(&self, input: &str) {
        let _ = input;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    VecU32(Vec<u32>),
    Usize(usize),
    String(String),
    Unsolved,
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

macro_rules! full_year {
    () => {
        day! {
            1::day01::DayOne,
            2::day02::DayTwo,
            3::day03::DayThree,
            4::day04::DayFour,
            5::day05::DayFive,
            6::day06::DaySix,
            7::day07::DaySeven,
            8::day08::DayEight,
            9::day09::DayNine,
            10::day10::DayTen,
            11::day11::DayEleven,
            12::day12::DayTwelve,
            13::day13::DayThirteen,
            14::day14::DayFourteen,
            15::day15::DayFifteen,
            16::day16::DaySixteen,
            17::day17::DaySeventeen,
            18::day18::DayEighteen,
            19::day19::DayNineteen,
            20::day20::DayTwenty,
            21::day21::DayTwentyOne,
            22::day22::DayTwentyTwo,
            23::day23::DayTwentyThree,
            24::day24::DayTwentyFour,
            25::day25::DayTwentyFive
        }
    };
}

pub mod year_2015 {
    //! Warm up for 2022
    full_year! {}
}

pub mod year_2022 {
    full_year! {}
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::I8(v) => v.fmt(f),
            Solution::I16(v) => v.fmt(f),
            Solution::I32(v) => v.fmt(f),
            Solution::U8(v) => v.fmt(f),
            Solution::U16(v) => v.fmt(f),
            Solution::U32(v) => v.fmt(f),
            Solution::U64(v) => v.fmt(f),
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
from! {i8, I8}
from! {i16, I16}
from! {i32, I32}
from! {u8, U8}
from! {u16, U16}
from! {u32, U32}
from! {u64, U64}
from! {String, String}

impl From<&str> for Solution {
    fn from(v: &str) -> Self {
        Self::String(v.to_owned())
    }
}

impl From<Vec<u32>> for Solution {
    fn from(v: Vec<u32>) -> Self {
        Self::VecU32(v)
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! test_impl {
    ($struct:ident, $solution1:tt, $solution2:tt, $input:tt) => {
        const INPUT: &str = $input;
        #[test]
        fn test_part1() {
            let answer: Solution = $solution1.into();
            let day = $struct {};
            assert_eq!(day.part_one(INPUT), answer);
        }
        #[test]
        fn test_part2() {
            let answer: Solution = $solution2.into();
            let day = $struct {};
            assert_eq!(day.part_two(INPUT), answer);
        }
    };
}
#[macro_export]
#[cfg(test)]
macro_rules! test {
    ($struct:ident, $solution1:tt, $solution2:tt, $input:tt) => {
        use $crate::test_impl;
        test_impl! {$struct, $solution1, $solution2, $input}
    };
    ($struct:ident, $solution1:tt, $input:tt) => {
        use $crate::{test_impl, Solution::Unsolved};
        test_impl! {$struct, $solution1, Unsolved, $input}
    };
    ($struct:ident, $input:tt) => {
        use $crate::{test_impl, Solution::Unsolved};
        test_impl! {$struct, Unsolved, Unsolved, $input}
    };
}

#[macro_export]
#[cfg(test)]
/// `bench {2022, 1, DayOne, year_2022, (answer_1), (answer_2)}`
macro_rules! bench {
    ($year:literal, $day:tt, $struct:ident) => {
        use test::{black_box, Bencher};

        #[bench]
        fn bench_part1(b: &mut Bencher) {
            use std::fs;
            let content =
                fs::read_to_string(format!("./data/{}/day{:0>2}.txt", $year, $day)).unwrap();
            let day = $struct {};
            b.iter(|| {
                day.part_one(black_box(&content));
            })
        }

        #[bench]
        fn bench_part2(b: &mut Bencher) {
            use std::fs;
            let content =
                fs::read_to_string(format!("./data/{}/day{:0>2}.txt", $year, $day)).unwrap();
            let day = $struct {};
            b.iter(|| {
                day.part_two(black_box(&content));
            })
        }
    };
    ($year:literal, $day:tt, $struct:ident, $answer_1:expr, $answer_2:expr) => {
        use test::{black_box, Bencher};

        #[bench]
        fn bench_part1(b: &mut Bencher) {
            use std::fs;
            let content =
                fs::read_to_string(format!("./data/{}/day{:0>2}.txt", $year, $day)).unwrap();
            let day = $struct {};
            b.iter(|| {
                black_box(day.part_one(black_box(&content)));
            });
            let answer: Solution = $answer_1.into();
            let result = day.part_one(&content);
            assert_eq!(answer, result);
        }
        #[bench]
        fn bench_part2(b: &mut Bencher) {
            use std::fs;
            let content =
                fs::read_to_string(format!("./data/{}/day{:0>2}.txt", $year, $day)).unwrap();
            let day = $struct {};
            b.iter(|| {
                black_box(day.part_two(black_box(&content)));
            });
            let answer: Solution = $answer_2.into();
            let result = day.part_two(&content);
            assert_eq!(answer, result);
        }
    };
}
