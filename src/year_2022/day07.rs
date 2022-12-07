use std::{iter::Peekable, str::Lines};

#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};
pub struct DaySeven {}

// $ - command
//  cd (x, .., /)
//  ls
// 123 abc (file abc has size 123)
// dir xyz (cwd contains directory xyz)

fn find_size(sum: &mut u32, lines: &mut Peekable<Lines<'_>>) -> u32 {
    let mut dir_sum = 0;
    while let Some(line) = lines.next() {
        match line {
            "$ cd .." => break,
            "$ ls" => loop {
                match lines.peek() {
                    Some(v) if v.starts_with('$') => break,
                    None => break,
                    _ => {}
                }
                let l = lines.next().unwrap();
                if !l.starts_with("dir") {
                    let (v, _) = l.split_once(' ').unwrap();
                    dir_sum += v.parse::<u32>().unwrap();
                }
            },
            _ => {
                dir_sum += find_size(sum, lines);
            }
        }
    }
    if dir_sum <= 100_000 {
        *sum += dir_sum;
    }
    dir_sum
}

fn find_small(sum: &mut u32, sum_of_dir: &mut Vec<u32>, lines: &mut Peekable<Lines<'_>>) -> u32 {
    let mut dir_sum = 0;
    while let Some(line) = lines.next() {
        match line {
            "$ cd .." => break,
            "$ ls" => loop {
                match lines.peek() {
                    Some(v) if v.starts_with('$') => break,
                    None => break,
                    _ => {}
                }
                let l = lines.next().unwrap();
                if !l.starts_with("dir") {
                    let (v, _) = l.split_once(' ').unwrap();
                    let val = v.parse::<u32>().unwrap();
                    dir_sum += val;
                    *sum += val;
                }
            },
            _ => {
                dir_sum += find_small(sum, sum_of_dir, lines);
            }
        }
    }
    sum_of_dir.push(dir_sum);
    dir_sum
}

#[allow(unused_must_use)]
impl AdventSolver for DaySeven {
    fn part_one(&self, input: &str) -> Solution {
        let mut lines = input.lines().peekable();
        // let mut iter = Parser(input.as_bytes().iter());
        let mut sum = 0;
        find_size(&mut sum, &mut lines);
        sum.into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let mut lines = input.lines().peekable();
        let mut sum = 0;
        let mut sum_of_dir = Vec::with_capacity(200);
        find_small(&mut sum, &mut sum_of_dir, &mut lines);
        let space_needed = 70_000_000 - 30_000_000;
        let mut smallest = 30_000_000;
        for dir in sum_of_dir {
            if (sum - dir) < space_needed && dir < smallest {
                smallest = dir;
            }
        }
        // panic!();
        smallest.into()
    }
}

#[cfg(test)]
bench! {2022, 7, DaySeven, year_2022, Solution::U32(1_749_646), Solution::U32(1_498_966)}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn part1() {
        let answer: Solution = 95437_u32.into();
        let day = DaySeven {};
        assert_eq!(day.part_one(INPUT), answer);
    }
    #[test]
    fn part2() {
        let answer: Solution = 24_933_642_u32.into();
        let day = DaySeven {};
        assert_eq!(day.part_two(INPUT), answer);
    }
}
