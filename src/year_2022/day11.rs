use arrayvec::ArrayVec;

use crate::prelude::*;
pub struct DayEleven {}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

#[derive(Debug, Clone, Copy)]
struct Monkey {
    pub op: Op,
    pub diviser: u64,
    pub if_true: usize,
    pub if_false: usize,
}

#[inline(always)]
fn collect_monkeys(input: &str) -> (ArrayVec<Monkey, 8>, ArrayVec<(u64, usize), 64>) {
    let mut iter = input.as_bytes().iter();
    let mut monkeys = ArrayVec::<Monkey, 8>::new();
    let mut items = ArrayVec::<(u64, usize), 64>::new();

    unsafe {
        for i in 0..8 {
            iter.nth(27).unwrap_unchecked();
            let val = ((iter.next().unwrap_unchecked() & 0x0f) * 10
                + (iter.next().unwrap_unchecked() & 0x0f)) as u64;
            items.push((val, i));

            loop {
                let next_byte = iter.next().unwrap_unchecked();
                if *next_byte == b',' {
                    let val = ((iter.nth(1).unwrap_unchecked() & 0x0f) * 10
                        + (iter.next().unwrap_unchecked() & 0x0f))
                        as u64;
                    items.push((val, i));
                } else {
                    break;
                }
            }

            // Collect op
            let op = iter.nth(23).unwrap_unchecked();
            let op_val = iter.nth(1).unwrap_unchecked();
            let op = if *op_val == b'o' {
                // Skip?
                iter.nth(2).unwrap_unchecked();
                Op::Square
            } else if *op == b'*' {
                let v_2 = iter.next().unwrap_unchecked();
                if *v_2 == b'\n' {
                    Op::Mul((op_val & 0x0f) as u64)
                } else {
                    iter.next().unwrap_unchecked();
                    Op::Mul(((op_val & 0x0f) * 10 + (v_2 & 0x0f)) as u64)
                }
            } else {
                let v_2 = iter.next().unwrap_unchecked();
                if *v_2 == b'\n' {
                    Op::Add((op_val & 0x0f) as u64)
                } else {
                    iter.next().unwrap_unchecked();
                    Op::Add(((op_val & 0x0f) * 10 + (v_2 & 0x0f)) as u64)
                }
            };

            // Collect Test
            let div_1 = iter.nth(21).unwrap_unchecked();
            let div_2 = iter.next().unwrap_unchecked();
            let diviser = if *div_2 == b'\n' {
                (div_1 & 0x0f) as u64
            } else {
                iter.next().unwrap_unchecked();
                ((div_1 & 0x0f) * 10 + (div_2 & 0x0f)) as u64
            };

            let if_true = (iter.nth(29).unwrap_unchecked() & 0x0f) as usize;
            let if_false = (iter.nth(31).unwrap_unchecked() & 0x0f) as usize;

            iter.next().unwrap_unchecked();
            iter.next(); //.unwrap_unchecked();
            monkeys.push(Monkey {
                op,
                diviser,
                if_true,
                if_false,
            });
        }
    }
    (monkeys, items)
}

impl AdventSolver for DayEleven {
    fn part_one(&self, input: &str) -> Solution {
        let (monkeys, mut items) = collect_monkeys(input);
        let mut inspections = [0_u64; 8];
        for _ in 0..20 {
            for item in &mut items {
                loop {
                    let monkey = unsafe { monkeys.get_unchecked(item.1) };
                    inspections[item.1] += 1;
                    item.0 = match monkey.op {
                        Op::Add(v) => item.0 + v,
                        Op::Mul(v) => item.0 * v,
                        Op::Square => item.0 * item.0,
                    };
                    item.0 /= 3;
                    let next = if item.0 % monkey.diviser == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };
                    if next < item.1 {
                        item.1 = next;
                        break;
                    }
                    item.1 = next;
                }
            }
        }
        inspections.sort_unstable();
        inspections.reverse();
        (inspections[0] * inspections[1]).into()
    }

    fn part_two(&self, input: &str) -> Solution {
        const MODULO: u64 = 13 * 3 * 7 * 2 * 19 * 5 * 11 * 17;
        const ROUNDS: usize = 10_000;
        let (monkeys, mut items) = collect_monkeys(input);
        let mut inspections = [0_u64; 8];
        let f = |item: &mut (u64, usize), round: &mut usize, inspections: &mut [u64; 8]| {
            let monkey = unsafe { monkeys.get_unchecked(item.1) };
            *unsafe { inspections.get_unchecked_mut(item.1) } += 1;
            item.0 = match monkey.op {
                Op::Add(v) => item.0 + v,
                Op::Mul(v) => item.0 * v,
                Op::Square => item.0 * item.0,
            };

            item.0 %= MODULO;
            item.1 = if item.0 % monkey.diviser == 0 {
                if item.1 > monkey.if_true {
                    *round += 1;
                }
                monkey.if_true
            } else {
                if item.1 > monkey.if_false {
                    *round += 1;
                }
                monkey.if_false
            };
        };
        for item in &mut items {
            let mut checkpoint = *item;
            let mut cycle_length = 1;
            let mut cycle = 0;
            let mut round = 0;
            while round < ROUNDS {
                for _ in 0..cycle_length {
                    cycle += 1;
                    f(item, &mut round, &mut inspections);
                    if checkpoint == *item {
                        let mut cycle_inspections = [0_u64; 8];
                        let mut cycle_round = 0;
                        for _ in 0..cycle {
                            f(item, &mut cycle_round, &mut cycle_inspections);
                        }
                        let amount = (ROUNDS - round) / cycle_round;
                        for i in 0..inspections.len() {
                            inspections[i] += cycle_inspections[i] * amount as u64;
                        }
                        round += amount * cycle_round;
                        while round < ROUNDS {
                            f(item, &mut round, &mut inspections);
                        }
                    }
                    if round == ROUNDS {
                        break;
                    }
                }
                cycle = 0;
                cycle_length *= 2;
                checkpoint = *item;
            }
        }

        inspections.sort_unstable();
        inspections.reverse();
        (inspections[0] * inspections[1]).into()
    }
}

#[cfg(test)]
bench! {2022, 11, DayEleven, 119_715_u64, 18_085_004_878_u64}

#[cfg(test)]
test! {DayEleven, 10605_u64, 2_713_310_158_u64, "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"}
