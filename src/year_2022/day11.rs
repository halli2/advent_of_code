use std::thread;

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
        let (mut monkeys, mut items) = collect_monkeys(input);
        let mut inspections = [0_u64; 8];
        for _ in 0..20 {
            for item in &mut items {
                loop {
                    let monkey = unsafe { monkeys.get_unchecked_mut(item.1) };
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
        let (monkeys, items) = collect_monkeys(input);
        let handles = items
            .into_iter()
            .map(|mut i| {
                let monkeys = monkeys.clone();
                thread::spawn(move || -> [u64; 8] {
                    let mut inspections = [0_u64; 8];
                    let mut round = 0;
                    while round < 10_000 {
                        let monkey = unsafe { monkeys.get_unchecked(i.1) };
                        *unsafe { inspections.get_unchecked_mut(i.1) } += 1;
                        i.0 = unsafe {
                            match monkey.op {
                                Op::Add(v) => i.0.unchecked_add(v),
                                Op::Mul(v) => i.0.unchecked_mul(v),
                                Op::Square => i.0.unchecked_mul(i.0),
                            }
                        };

                        i.0 %= MODULO;
                        i.1 = if i.0 % monkey.diviser == 0 {
                            if i.1 > monkey.if_true {
                                round += 1;
                            }
                            monkey.if_true
                        } else {
                            if i.1 > monkey.if_false {
                                round += 1;
                            }
                            monkey.if_false
                        };
                    }
                    inspections
                })
            })
            .collect::<Vec<_>>();

        let mut inspections = [0_u64; 8];
        for handle in handles {
            let handle_res = handle.join().unwrap();
            for i in 0..8 {
                inspections[i] += handle_res[i];
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
