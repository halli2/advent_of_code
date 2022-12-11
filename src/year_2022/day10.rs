use crate::prelude::*;
pub struct DayTen {}

fn iterate<F: FnMut(i32)>(input: &str, f: &mut F) {
    let mut instr = input.as_bytes().iter();
    let mut register = 1;
    while let Some(op) = instr.next() {
        if *op == b'a' {
            // Add
            let value = match *instr.nth(4).unwrap() {
                // Negative number
                val if val == b'-' => {
                    let mut v = -((instr.next().unwrap() & 0x0f) as i8);
                    let next = instr.next().unwrap();
                    if *next != b'\n' {
                        v = v * 10 - (next & 0x0f) as i8;
                        instr.next().unwrap();
                    }
                    v
                }
                // Positive number
                val => {
                    let mut v = (val & 0x0f) as i8;
                    let next = instr.next().unwrap();
                    if *next != b'\n' {
                        v = v * 10 + (next & 0x0f) as i8;
                        instr.next().unwrap();
                    }
                    v
                }
            };
            f(register);
            f(register);
            register += value as i32;
        } else {
            // Noop
            instr.nth(3).unwrap();
            f(register);
        }
    }
}

impl AdventSolver for DayTen {
    fn part_one(&self, input: &str) -> Solution {
        let mut cycle = 20;
        let mut res: i32 = 0;

        let mut f = |register: i32| {
            cycle += 1;
            if cycle % 40 == 0 {
                res += (cycle - 20) * register;
            }
        };

        iterate(input, &mut f);
        res.into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let mut cycle = 0;
        let mut pixels = String::with_capacity(256);

        let mut f = |register: i32| {
            if (cycle - register).abs() < 2 {
                pixels.push('#');
            } else {
                pixels.push(' ');
            }
            cycle += 1;
            if cycle == 40 {
                pixels.push('\n');
                cycle = 0;
            }
        };

        iterate(input, &mut f);
        pixels.into()
    }
}

#[cfg(test)]
bench! {2022, 10, DayTen, 12560_i32}
#[cfg(test)]
test! {DayTen, 13140_i32,
"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"}
