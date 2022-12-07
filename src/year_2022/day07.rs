use smallvec::SmallVec;

#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};
pub struct DaySeven {}

// $ - command
//  cd (x, .., /)
//  ls
// 123 abc (file abc has size 123)
// dir xyz (cwd contains directory xyz)

struct Parser<'a, T: ExactSizeIterator<Item = &'a u8>>(T);

impl<'a, T: ExactSizeIterator<Item = &'a u8>> Parser<'a, T> {
    const fn new(iter: T) -> Self {
        Self(iter)
    }

    #[inline(always)]
    fn ln(&mut self) {
        for v in self.0.by_ref() {
            if *v == b'\n' {
                break;
            }
        }
    }
}

#[derive(Debug)]
enum Cmd {
    Break,
    NewDir,
    Pass,
    Size(u32),
}

impl<'a, T: ExactSizeIterator<Item = &'a u8>> Iterator for Parser<'a, T> {
    type Item = Cmd;

    fn next(&mut self) -> Option<Self::Item> {
        let byte_1 = self.0.next()?;
        match byte_1 {
            b'$' => {
                let cmd = match self.0.nth(1)? {
                    b'c' => {
                        if *self.0.nth(2)? == b'.' {
                            self.0.nth(1)?;
                            Cmd::Break
                        } else {
                            self.ln();
                            Cmd::NewDir
                        }
                    }
                    b'l' => {
                        self.0.nth(1)?;
                        Cmd::Pass
                    }
                    _ => unreachable!(),
                };
                Some(cmd)
            } // command
            b'd' => {
                self.ln();
                Some(Cmd::Pass)
            } // dir
            _ => {
                let mut size = u32::from(byte_1 & 0x0f);
                loop {
                    let next_byte = self.0.next()?;
                    if *next_byte == b' ' {
                        break;
                    }
                    size = size
                        .wrapping_mul(10)
                        .wrapping_add(u32::from(next_byte & 0x0f));
                }
                // iter till end
                self.ln();
                Some(Cmd::Size(size))
            } // size
        }
    }
}

fn parse<'a, T: ExactSizeIterator<Item = &'a u8>>(
    sum: &mut u32,
    parser: &mut Parser<'a, T>,
) -> u32 {
    let mut directory = 0;
    while let Some(cmd) = parser.next() {
        match cmd {
            Cmd::Break => break,
            Cmd::NewDir => directory += parse(sum, parser),
            Cmd::Pass => {}
            Cmd::Size(v) => directory += v,
        }
    }
    if directory <= 100_000 {
        *sum += directory;
    }
    directory
}

fn parse_small<'a, T: ExactSizeIterator<Item = &'a u8>>(
    sum: &mut u32,
    sum_of_dir: &mut SmallVec<[u32; 256]>,
    parser: &mut Parser<'a, T>,
) -> u32 {
    let mut directory = 0;
    while let Some(cmd) = parser.next() {
        match cmd {
            Cmd::Break => break,
            Cmd::NewDir => directory += parse_small(sum, sum_of_dir, parser),
            Cmd::Pass => {}
            Cmd::Size(v) => {
                directory += v;
                *sum += v;
            }
        }
    }
    sum_of_dir.push(directory);
    directory
}

impl AdventSolver for DaySeven {
    fn part_one(&self, input: &str) -> Solution {
        let mut parser = Parser::new(input.as_bytes().iter());
        let mut sum = 0;
        parse(&mut sum, &mut parser);
        sum.into()
    }
    fn part_two(&self, input: &str) -> Solution {
        let mut parser = Parser::new(input.as_bytes().iter());
        let mut sum = 0;
        let mut sum_of_dir = SmallVec::new();
        parse_small(&mut sum, &mut sum_of_dir, &mut parser);
        let space_needed = 70_000_000 - 30_000_000;
        let mut smallest = 30_000_000;
        for dir in sum_of_dir {
            if (sum - dir) < space_needed && dir < smallest {
                smallest = dir;
            }
        }
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
