use crate::prelude::*;
pub struct DayNine {}

struct Parser<'a, T: ExactSizeIterator<Item = &'a u8>>(T);

impl<'a, T: ExactSizeIterator<Item = &'a u8>> Iterator for Parser<'a, T> {
    // (x, y, n)
    type Item = (i8, i8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let dir_byte = unsafe { self.0.next().unwrap_unchecked() };
        // skip ' '
        unsafe { self.0.next().unwrap_unchecked() };

        // get count
        let mut value = unsafe { self.0.next().unwrap_unchecked() } & 0x0f;
        let value_2 = unsafe { self.0.next().unwrap_unchecked() };
        if *value_2 == b'\n' {
        } else {
            value = unsafe { value.unchecked_mul(10).unchecked_add(value_2 & 0x0f) };
            unsafe { self.0.next().unwrap_unchecked() };
        }
        Some(match dir_byte {
            b'R' => (1, 0, value),
            b'L' => (-1, 0, value),
            b'U' => (0, 1, value),
            b'D' => (0, -1, value),
            _ => unreachable!(),
        })
    }
}

impl AdventSolver for DayNine {
    fn part_one(&self, input: &str) -> Solution {
        let motions = input.as_bytes();
        let mut visited = Vec::with_capacity(2048);
        // x, y
        let mut head = (0, 0);
        let mut tail = (0, 0);
        visited.push(tail);

        Parser(motions.iter()).for_each(|motion| {
            head.0 += unsafe { i32::from(motion.0.unchecked_mul(motion.2 as i8)) };
            head.1 += unsafe { i32::from(motion.1.unchecked_mul(motion.2 as i8)) };
            loop {
                let (x, y) = (head.0 - tail.0, head.1 - tail.1);
                if x.abs() > 1 || y.abs() > 1 {
                    tail.0 += x.signum();
                    tail.1 += y.signum();
                    visited.push(tail);
                } else {
                    break;
                }
            }
        });

        visited.sort_unstable();
        visited.dedup();
        visited.len().into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let motions = input.as_bytes();
        let mut visited = Vec::with_capacity(2048);

        // index 0 is head, index 9 is tails
        let mut knots = [(0, 0); 10];
        visited.push(knots[9]);

        Parser(motions.iter()).for_each(|motion| {
            knots[0].0 += unsafe { i32::from(motion.0.unchecked_mul(motion.2 as i8)) };
            knots[0].1 += unsafe { i32::from(motion.1.unchecked_mul(motion.2 as i8)) };
            for _ in 0..motion.2 {
                let old = unsafe { knots.last().copied().unwrap_unchecked() };
                for i in 1..knots.len() {
                    let (x, y) = (knots[i - 1].0 - knots[i].0, knots[i - 1].1 - knots[i].1);
                    if x.abs() > 1 || y.abs() > 1 {
                        knots[i].0 += x.signum();
                        knots[i].1 += y.signum();
                    }
                }
                let new = unsafe { knots.last().copied().unwrap_unchecked() };
                if new != old {
                    visited.push(new);
                }
            }
        });

        visited.sort_unstable();
        visited.dedup();
        visited.len().into()
    }
}

#[cfg(test)]
bench! {2022, 9, DayNine, 5960_usize, 2327_usize}

#[cfg(test)]
test! {DayNine, 13_usize, 36_usize, "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
", "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"}
