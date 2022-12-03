#[cfg(test)]
use crate::bench;
use crate::{AdventSolver, Solution};
pub struct DayThree {}

// A-Z 65-90
// a-z 97-122
// A - 38 = 27
// a - 38 = 59 => result = (x - 38) % 58

impl AdventSolver for DayThree {
    fn part_one(&self, input: &str) -> Solution {
        let input = input.lines();
        let mut total = 0_u32;
        for line in input {
            let l = line.as_bytes();
            let mid = l.len() / 2;
            let left = &l[..mid];
            let right = &l[mid..];
            let dup = left.iter().find(|x| right.iter().any(|y| y == *x)).unwrap();
            total += u32::from((dup - 38) % 58);
        }
        total.into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let input = input
            .split_whitespace()
            .map(str::as_bytes)
            .collect::<Vec<_>>();
        let mut dup_vec: Vec<u8> = Vec::new();
        let mut total = 0;
        for chunk in input.chunks(3) {
            dup_vec.clear();
            dup_vec.extend(chunk[0].iter().filter(|x| chunk[1].iter().any(|y| y == *x)));
            let dup = chunk[2]
                .iter()
                .find(|x| dup_vec.iter().any(|y| *x == y))
                .unwrap();
            total += u32::from((dup - 38) % 58);
        }
        total.into()
    }
}

#[cfg(test)]
bench! {2022, 3, DayThree, year_2022, Solution::U32(7824), Solution::U32(2798)}

#[cfg(test)]
mod teests {
    #[test]
    fn ggg() {
        let a = 97;
        let cap_a = 65;
        assert_eq!(1, (a - 38) % 58);
        assert_eq!(27, (cap_a - 38) % 58);
    }
}
