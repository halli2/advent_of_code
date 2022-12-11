use smallvec::SmallVec;

use crate::prelude::*;
pub struct DayEleven {}

// New = old <op> <u32>
#[derive(Debug)]
enum Op {
    Add(u64),
    Mul(u64),
    AddOld,
    MulOld,
}

#[derive(Debug)]
struct Monkey {
    pub items: SmallVec<[u64; 32]>,
    pub op: Op,
    pub test: (u64, usize, usize),
    pub inspections: u64,
}

impl AdventSolver for DayEleven {
    fn part_one(&self, input: &str) -> Solution {
        let mut monkeys: SmallVec<[Monkey; 7]> = SmallVec::new();
        // let mut monkeys: Vec<Vec<u32>> = Vec::default();
        // let mut ops: SmallVec<[Op; 7]> = SmallVec::new();
        // test: true(monkey), false(monkey)
        // let mut tests: SmallVec<[(u32, usize, usize); 7]> = SmallVec::new();
        let input = input.to_owned() + "\n";
        for chunk in input.lines().array_chunks::<7>() {
            // Push items
            let items = chunk[1]
                .split_once(':')
                .map(|(_, nums)| {
                    nums.split_whitespace()
                        .into_iter()
                        .map(|num| num.replace(',', "").parse::<u64>().unwrap())
                })
                .unwrap()
                .collect::<SmallVec<[u64; 32]>>();
            let (_, op) = chunk[2].split_once("old ").unwrap();
            let (op, val) = op.split_once(' ').unwrap();
            let op = match op {
                "+" => {
                    if val == "old" {
                        Op::AddOld
                    } else {
                        Op::Add(val.parse::<u64>().unwrap())
                    }
                }
                "*" => {
                    if val == "old" {
                        Op::MulOld
                    } else {
                        Op::Mul(val.parse::<u64>().unwrap())
                    }
                }
                _ => unreachable!(),
            };

            let test = chunk[3]
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let if_true = chunk[4]
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let if_false = chunk[5]
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let test = (test, if_true, if_false);
            monkeys.push(Monkey {
                items,
                op,
                test,
                inspections: 0,
            });
        }

        // 20 rounds
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                // Need 2 mutable monkeys
                let monkey = unsafe { &mut *std::ptr::addr_of_mut!(monkeys[i]) };
                monkey.items.reverse();
                for _ in 0..monkey.items.len() {
                    monkey.inspections += 1;
                    let mut level = monkey.items.pop().unwrap();
                    level = match monkey.op {
                        Op::Add(v) => level + v,
                        Op::Mul(v) => level * v,
                        Op::AddOld => level + level,
                        Op::MulOld => level * level,
                    };
                    level /= 3;
                    // test
                    let test = monkey.test;
                    if level % test.0 == 0 {
                        monkeys[test.1].items.push(level);
                    } else {
                        monkeys[test.2].items.push(level);
                    }
                    // &mut *std::ptr::addr_of_mut!(arr[b]),
                }
            }
        }

        let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();
        inspections.sort_unstable();
        inspections.reverse();
        (inspections[0] * inspections[1]).into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let mut monkeys: SmallVec<[Monkey; 7]> = SmallVec::new();
        // let mut monkeys: Vec<Vec<u32>> = Vec::default();
        // let mut ops: SmallVec<[Op; 7]> = SmallVec::new();
        // test: true(monkey), false(monkey)
        // let mut tests: SmallVec<[(u32, usize, usize); 7]> = SmallVec::new();
        let input = input.to_owned() + "\n";
        for chunk in input.lines().array_chunks::<7>() {
            // Push items
            let items = chunk[1]
                .split_once(':')
                .map(|(_, nums)| {
                    nums.split_whitespace()
                        .into_iter()
                        .map(|num| num.replace(',', "").parse::<u64>().unwrap())
                })
                .unwrap()
                .collect::<SmallVec<[u64; 32]>>();
            let (_, op) = chunk[2].split_once("old ").unwrap();
            let (op, val) = op.split_once(' ').unwrap();
            let op = match op {
                "+" => {
                    if val == "old" {
                        Op::AddOld
                    } else {
                        Op::Add(val.parse::<u64>().unwrap())
                    }
                }
                "*" => {
                    if val == "old" {
                        Op::MulOld
                    } else {
                        Op::Mul(val.parse::<u64>().unwrap())
                    }
                }
                _ => unreachable!(),
            };

            let test = chunk[3]
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let if_true = chunk[4]
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let if_false = chunk[5]
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let test = (test, if_true, if_false);
            monkeys.push(Monkey {
                items,
                op,
                test,
                inspections: 0,
            });
        }

        let modulu: u64 = monkeys.iter().map(|m| m.test.0).product();

        // xd
        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                // Need 2 mutable monkeys
                let monkey = unsafe { &mut *std::ptr::addr_of_mut!(monkeys[i]) };
                monkey.items.reverse();
                for _ in 0..monkey.items.len() {
                    monkey.inspections += 1;
                    let mut level = monkey.items.pop().unwrap();
                    level = match monkey.op {
                        Op::Add(v) => level + v,
                        Op::Mul(v) => level * v,
                        Op::AddOld => level + level,
                        Op::MulOld => level * level,
                    };

                    level %= modulu;
                    // level /= 3;
                    // test
                    let test = monkey.test;
                    if level % test.0 == 0 {
                        monkeys[test.1].items.push(level);
                    } else {
                        monkeys[test.2].items.push(level);
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
