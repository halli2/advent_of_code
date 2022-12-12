use std::collections::VecDeque;

use crate::prelude::*;
pub struct DayTwelve {}

impl AdventSolver for DayTwelve {
    fn part_one(&self, input: &str) -> Solution {
        let mut grid = Vec::new();
        let mut start = (0, 0);
        let mut finish = (0, 0);
        // Lol yikes
        for (i, line) in input.lines().enumerate() {
            grid.push(Vec::new());
            for (j, b) in line.as_bytes().iter().enumerate() {
                if *b == b'S' {
                    start = (i, j);
                    grid[i].push(b'a');
                } else if *b == b'E' {
                    finish = (i, j);
                    grid[i].push(b'z');
                } else {
                    grid[i].push(*b);
                }
            }
        }

        let rows = grid.len();
        let cols = grid.first().unwrap().len();
        let mut visited = Vec::new();
        let mut queue = VecDeque::new();
        visited.push(start);
        queue.push_back((start, 1_u32));
        loop {
            let ((row, col), steps) = unsafe { queue.pop_front().unwrap_unchecked() };
            let mut neighbors = Vec::with_capacity(4);
            if row + 1 < rows {
                neighbors.push((row + 1, col));
            }
            if 0 < row {
                neighbors.push((row - 1, col));
            }
            if col + 1 < cols {
                neighbors.push((row, col + 1));
            }
            if 0 < col {
                neighbors.push((row, col - 1));
            }
            for neighbour in neighbors {
                // At most 1 up, but can fall as much as wanted...
                if grid[row][col] + 2 > grid[neighbour.0][neighbour.1]
                    && !visited.contains(&neighbour)
                {
                    if neighbour == finish {
                        return steps.into();
                    }
                    visited.push(neighbour);
                    queue.push_back((neighbour, steps + 1));
                }
            }
        }
    }

    fn part_two(&self, input: &str) -> Solution {
        let mut grid = Vec::new();
        let mut finish = (0, 0);
        // Lol yikes
        for (i, line) in input.lines().enumerate() {
            grid.push(Vec::new());
            for (j, b) in line.as_bytes().iter().enumerate() {
                if *b == b'E' {
                    finish = (i, j);
                    grid[i].push(b'z');
                } else {
                    grid[i].push(*b);
                }
            }
        }

        let rows = grid.len();
        let cols = grid.first().unwrap().len();
        let mut visited = Vec::new();
        let mut queue = VecDeque::new();
        visited.push(finish);
        queue.push_back((finish, 1_u32));
        while let Some(((row, col), steps)) = queue.pop_front() {
            let mut neighbors = Vec::with_capacity(4);
            if row + 1 < rows {
                neighbors.push((row + 1, col));
            }
            if 0 < row {
                neighbors.push((row - 1, col));
            }
            if col + 1 < cols {
                neighbors.push((row, col + 1));
            }
            if 0 < col {
                neighbors.push((row, col - 1));
            }
            for neighbour in neighbors {
                // At most 1 up, but can fall as much as wanted...
                if grid[row][col] < grid[neighbour.0][neighbour.1] + 2
                    && !visited.contains(&neighbour)
                {
                    if grid[neighbour.0][neighbour.1] == b'a' {
                        return steps.into();
                    }
                    visited.push(neighbour);
                    queue.push_back((neighbour, steps + 1));
                }
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
bench! {2022, 12, DayTwelve, 394_u32, 388_u32}

#[cfg(test)]
test! {DayTwelve, 31_u32, 29_u32,  "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"}
