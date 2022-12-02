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
bench! {2022, 1, DayOne, year_2022, Solution::U32(68292), Solution::U32(203203)}
