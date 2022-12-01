use crate::{AdventSolver, Solution};
pub struct DayOne {}

impl AdventSolver for DayOne {
    fn part_one(&self, input: &str) -> Solution {
        let mut highest = 0;
        let mut counter = 0;
        for line in input.lines() {
            match line.parse::<u32>() {
                Ok(v) => {
                    counter += v;
                }
                Err(_) => {
                    if counter > highest {
                        highest = counter;
                    }
                    counter = 0;
                }
            }
        }
        highest.into()
    }

    // fn part_two(&self, input: &str) -> Solution {
    //     let mut cals = Vec::new();
    //     let mut counter = 0;
    //     for line in input.lines() {
    //         match line.parse::<u32>() {
    //             Ok(v) => {
    //                 counter += v;
    //             }
    //             Err(_) => {
    //                 cals.push(counter);
    //                 counter = 0;
    //             }
    //         }
    //     }

    //     cals.sort();
    //     let mut res = 0;
    //     for _ in 0..3 {
    //         res += cals.pop().unwrap();
    //     }

    //     res.into()
    // }
    fn part_two(&self, input: &str) -> Solution {
        let mut cals = [0_u32; 3];
        let mut counter = 0;
        for line in input.lines() {
            match line.parse::<u32>() {
                Ok(v) => {
                    counter += v;
                }
                Err(_) => {
                    let smallest = cals.iter().min().unwrap();
                    if counter > *smallest {
                        if let Some(index) = cals.iter().position(|x| smallest == x) {
                            cals[index] = counter;
                        };
                    }
                    counter = 0;
                }
            }
        }
        cals.iter().sum::<u32>().into()
    }

    fn visualize(&self, _input: &str) {}
}
