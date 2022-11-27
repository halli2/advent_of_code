#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]

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
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
    fn visualize(&self, _input: &str) {}
}
