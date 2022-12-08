use core::slice;

#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};
pub struct DayEight {}

impl AdventSolver for DayEight {
    fn part_one(&self, input: &str) -> Solution {
        let trees = input.as_bytes();
        let len = slice::memchr::memchr(b'\n', trees).unwrap();
        let width = len + 1;
        let mut grid = [[false; 99]; 99];
        let mut highest_up: [u8; 99] = [0; 99];
        let mut highest_down: [u8; 99] = [0; 99];
        for i in 0..len {
            let mut highest_left = 0;
            let mut highest_right = 0;
            for j in 0..len {
                let tree = trees[i * width + j];
                if tree > highest_left {
                    grid[i][j] = true;
                    highest_left = tree;
                }
                if tree > highest_up[j] {
                    grid[i][j] = true;
                    highest_up[j] = tree;
                }

                let tree = trees[(len - i - 1) * width + j];
                if tree > highest_down[j] {
                    grid[len - i - 1][j] = true;
                    highest_down[j] = tree;
                }
                let tree = trees[i * width + len - j - 1];
                if tree > highest_right {
                    grid[i][len - j - 1] = true;
                    highest_right = tree;
                }
            }
        }

        grid.into_iter().flatten().filter(|&vis| vis).count().into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let trees = input.as_bytes();
        let len = slice::memchr::memchr(b'\n', trees).unwrap();
        let width = len + 1;
        let mut high_score = 0;
        for i in 0..len {
            for j in 0..len {
                let mut score: [u32; 4] = [0; 4];
                let mut prev = 0;
                let current_tree = trees[i * width + j];
                // sweep up
                for l in (0..i).rev() {
                    score[0] += 1;
                    let tree = trees[l * width + j];
                    if tree >= current_tree && tree > prev {
                        break;
                    }
                    prev = tree;
                }
                // Sweep down
                for l in (i + 1)..len {
                    score[1] += 1;
                    let tree = trees[l * width + j];
                    if tree >= current_tree && tree > prev {
                        break;
                    }
                    prev = tree;
                }
                // Sweep left
                for l in (0..j).rev() {
                    score[2] += 1;
                    let tree = trees[i * width + l];
                    if tree >= current_tree && tree > prev {
                        break;
                    }
                    prev = tree;
                }
                // Sweep right
                for l in (j + 1)..len {
                    score[3] += 1;
                    let tree = trees[i * width + l];
                    if tree >= current_tree && tree > prev {
                        break;
                    }
                    prev = tree;
                }

                let calc = score[0] * score[1] * score[2] * score[3];
                if high_score < calc {
                    high_score = calc;
                }
            }
        }
        high_score.into()
    }
}

#[cfg(test)]
bench! {2022, 8, DayEight, year_2022, Solution::Usize(1843), Solution::U32(180_000)}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "\
30373
25512
65332
33549
35390
";
    #[test]
    fn part1() {
        let answer: Solution = 21_usize.into();
        let day = DayEight {};
        assert_eq!(day.part_one(INPUT), answer);
    }
    #[test]
    fn part2() {
        let answer: Solution = 8_u32.into();
        let day = DayEight {};
        assert_eq!(day.part_two(INPUT), answer);
    }
}
