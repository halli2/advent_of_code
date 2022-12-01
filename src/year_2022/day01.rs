use crate::{AdventSolver, Solution};
pub struct DayOne {}

impl AdventSolver for DayOne {
    fn part_one(&self, input: &str) -> Solution {
        let mut highest = 0;
        let mut counter = 0;
        for line in input.lines() {
            if let Ok(v) = line.parse::<u32>() {
                counter += v;
            } else {
                if counter > highest {
                    highest = counter;
                }
                counter = 0;
            }
        }
        if counter > highest {
            highest = counter;
        }
        highest.into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let mut cals = [0_u32; 3];
        let mut counter = 0;
        for line in input.lines() {
            if let Ok(v) = line.parse::<u32>() {
                counter += v;
            } else {
                if let Some(smallest) = cals.iter().min() {
                    if counter > *smallest {
                        if let Some(index) = cals.iter().position(|x| smallest == x) {
                            cals[index] = counter;
                        };
                    }
                }
                counter = 0;
            }
        }
        let smallest = cals.iter().min().unwrap();
        if counter > *smallest {
            if let Some(index) = cals.iter().position(|x| smallest == x) {
                cals[index] = counter;
            };
        }
        cals.iter().sum::<u32>().into()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test::{black_box, Bencher};

    use crate::AdventSolver;

    use super::DayOne;

    #[bench]
    fn part1(b: &mut Bencher) {
        let content = fs::read_to_string("./data/2022/day01.txt").unwrap();
        let day = DayOne {};
        b.iter(|| {
            black_box(day.part_one(black_box(&content)));
        })
    }

    #[bench]
    fn part2(b: &mut Bencher) {
        let content = fs::read_to_string("./data/2022/day01.txt").unwrap();
        let day = DayOne {};
        b.iter(|| {
            black_box(day.part_two(black_box(&content)));
        })
    }
}
