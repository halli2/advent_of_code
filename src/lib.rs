mod error;
pub mod gpu;
pub mod parser;

pub mod year_2015 {
    //! Warm up for 2022

    pub mod day_1;
    pub use day_1::DayOne;
    pub mod day_2;
    pub use day_2::DayTwo;
    pub mod day_3;
    pub use day_3::DayThree;
}

pub mod year_2022 {
    pub mod day_0;
    pub mod day_2;

    pub use day_0::DayZero;
    pub use day_2::DayTwo;
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
