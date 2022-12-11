use crate::prelude::*;
pub struct DayEleven {}

// New = old <op> <u32>
#[derive(Debug)]
enum Op {
    Add(u64),
    Mul(u64),
    // AddOld,
    MulOld,
}

#[derive(Debug)]
struct Monkey {
    pub items: Vec<u64>,
    pub op: Op,
    pub diviser: u64,
    pub if_true: usize,
    pub if_false: usize,
    pub inspections: u64,
}

#[inline(always)]
fn collect_monkeys(input: &str) -> Vec<Monkey> {
    let mut iter = input.as_bytes().iter();
    let mut monkeys: Vec<Monkey> = Vec::with_capacity(8);

    unsafe {
        for _ in 0..8 {
            // Collect starting values
            let mut items = Vec::new();
            iter.nth(27).unwrap_unchecked();
            items.push(
                ((iter.next().unwrap_unchecked() & 0x0f) * 10
                    + (iter.next().unwrap_unchecked() & 0x0f)) as u64,
            );
            loop {
                let next_byte = iter.next().unwrap_unchecked();
                if *next_byte == b',' {
                    let val = ((iter.nth(1).unwrap_unchecked() & 0x0f) * 10
                        + (iter.next().unwrap_unchecked() & 0x0f))
                        as u64;
                    items.push(val);
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
                Op::MulOld
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
            iter.next().unwrap_unchecked();
            monkeys.push(Monkey {
                items,
                op,
                diviser,
                if_true,
                if_false,
                inspections: 0,
            });
        }
    }
    monkeys
}
impl AdventSolver for DayEleven {
    fn part_one(&self, input: &str) -> Solution {
        let mut monkeys = collect_monkeys(input);

        let monkey_len = monkeys.len();
        for _ in 0..20 {
            for i in 0..monkey_len {
                let monkey = unsafe { &mut *std::ptr::addr_of_mut!(monkeys[i]) };
                monkey.items.reverse();
                monkey.inspections += monkey.items.len() as u64;
                for _ in 0..monkey.items.len() {
                    let mut level = unsafe { monkey.items.pop().unwrap_unchecked() };
                    level = match monkey.op {
                        Op::Add(v) => level + v,
                        Op::Mul(v) => level * v,
                        Op::MulOld => level * level,
                    };
                    level /= 3;
                    if level % monkey.diviser == 0 {
                        monkeys[monkey.if_true].items.push(level);
                    } else {
                        monkeys[monkey.if_false].items.push(level);
                    }
                }
            }
        }
        let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();
        inspections.sort_unstable();
        inspections.reverse();
        (inspections[0] * inspections[1]).into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let mut monkeys = collect_monkeys(input);
        let modulo: u64 = monkeys.iter().map(|m| m.diviser).product();
        let monkey_len = monkeys.len();
        for _ in 0..10000 {
            for i in 0..monkey_len {
                let monkey = unsafe { &mut *std::ptr::addr_of_mut!(monkeys[i]) };
                // monkey.items.reverse();
                monkey.inspections += monkey.items.len() as u64;
                for _ in 0..monkey.items.len() {
                    let mut level = unsafe { monkey.items.pop().unwrap_unchecked() };
                    level = match monkey.op {
                        Op::Add(v) => level + v,
                        Op::Mul(v) => level * v,
                        Op::MulOld => level * level,
                    };
                    level %= modulo;
                    if level % monkey.diviser == 0 {
                        monkeys[monkey.if_true].items.push(level);
                    } else {
                        monkeys[monkey.if_false].items.push(level);
                    }
                }
            }
        }
        let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();
        inspections.sort_unstable();
        inspections.reverse();
        (inspections[0] * inspections[1]).into()
    }
}

#[cfg(test)]
bench! {2022, 11, DayEleven, 119_715_u64, 18_085_004_878_u64}

// #[cfg(test)]
// test! {DayEleven, 10605_u64, 2_713_310_158_u64, "\
// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3

// Monkey 1:
//   Starting items: 54, 65, 75, 74
//   Operation: new = old + 6
//   Test: divisible by 19
//     If true: throw to monkey 2
//     If false: throw to monkey 0

// Monkey 2:
//   Starting items: 79, 60, 97
//   Operation: new = old * old
//   Test: divisible by 13
//     If true: throw to monkey 1
//     If false: throw to monkey 3

// Monkey 3:
//   Starting items: 74
//   Operation: new = old + 3
//   Test: divisible by 17
//     If true: throw to monkey 0
//     If false: throw to monkey 1
// "}
