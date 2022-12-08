use core::slice;
use std::cell::Cell;

use crate::prelude::*;
pub struct DayEight {}

impl AdventSolver for DayEight {
    fn part_one(&self, input: &str) -> Solution {
        const N: usize = 99;
        let trees = input.as_bytes();
        let len = slice::memchr::memchr(b'\n', trees).unwrap();
        let width = len + 1;
        let mut grid = Array2D::new([false; N * N], N);
        let mut highest_up: [u8; N] = [0; N];
        let mut highest_down: [u8; N] = [0; N];
        for i in 0..len {
            let mut highest_left = 0;
            let mut highest_right = 0;
            for j in 0..len {
                let tree = trees[i * width + j];
                if tree > highest_left {
                    *grid.index_mut((i, j)) = true;
                    highest_left = tree;
                }
                if tree > highest_up[j] {
                    *grid.index_mut((i, j)) = true;
                    highest_up[j] = tree;
                }

                let tree = trees[(len - i - 1) * width + j];
                if tree > highest_down[j] {
                    *grid.index_mut((len - i - 1, j)) = true;
                    highest_down[j] = tree;
                }
                let tree = trees[i * width + len - j - 1];
                if tree > highest_right {
                    *grid.index_mut((i, len - j - 1)) = true;
                    highest_right = tree;
                }
            }
        }

        grid.inner.into_iter().filter(|&vis| vis).count().into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let trees = input.as_bytes();

        let len = trees.memchr(b'\n');
        slice::memchr::memchr(b'\n', trees).unwrap();
        let width = len + 1;
        let mut high_score = 0;
        for i in 0..len {
            for j in 0..len {
                let current_tree = trees[i * width + j];

                let done = Cell::new(false);
                let f = |tree: u8| -> bool {
                    if done.get() {
                        done.set(false);
                        return false;
                    }
                    if tree >= current_tree {
                        done.set(true);
                    }
                    true
                };
                // Sweep Up
                let mut score = (0..i)
                    .rev()
                    .take_while(|l| f(trees[*l * width + j]))
                    .count();
                // Sweep Down
                score *= ((i + 1)..len)
                    .take_while(|l| f(trees[*l * width + j]))
                    .count();
                // Sweep Left
                score *= (0..j)
                    .rev()
                    .take_while(|l| f(trees[i * width + *l]))
                    .count();
                // Sweep Right
                score *= ((j + 1)..len)
                    .take_while(|l| f(trees[i * width + *l]))
                    .count();

                high_score = high_score.max(score);
            }
        }
        high_score.into()
    }
}

#[cfg(test)]
bench! {2022, 8, DayEight, 1843_usize, 180_000_usize}

#[cfg(test)]
test! {DayEight, 21_usize, 8_usize, "\
30373
25512
65332
33549
35390
"}
