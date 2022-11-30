mod error;
pub mod gpu;
pub mod parser;

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
        2::day02::DayTwo
    }
}

pub trait AdventSolver {
    fn part_one(&self, input: &str) -> Solution;
    fn part_two(&self, input: &str) -> Solution;
    fn visualize(&self, _input: &str) {}
}

pub enum Solution {
    I32(i32),
    Usize(usize),
    String(String),
    Unsolved,
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::I32(v) => v.fmt(f),
            Solution::Usize(v) => v.fmt(f),
            Solution::String(v) => v.fmt(f),
            Solution::Unsolved => write!(f, "Something wrong happened, problem not solved"),
        }
    }
}

impl From<usize> for Solution {
    fn from(value: usize) -> Self {
        Self::Usize(value)
    }
}

impl From<i32> for Solution {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<String> for Solution {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
