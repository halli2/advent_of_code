#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]

mod error;
pub(crate) mod parser;
mod year_2022 {
    pub mod day_0;
    pub mod day_2;
}

pub use error::{Error, Result};
pub use year_2022::{day_0::DayZero, day_2::DayTwo};

pub trait AdventSolver {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}
