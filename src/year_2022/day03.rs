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
        input
            .lines()
            .map(str::as_bytes)
            .map(|l| {
                let mid = l.len() / 2;
                let left = &l[..mid];
                let right = &l[mid..];
                let dup = left.iter().find(|x| right.contains(x)).unwrap();
                u32::from((dup - 38) % 58)
            })
            .sum::<u32>()
            .into()
    }

    fn part_two(&self, input: &str) -> Solution {
        input
            .lines()
            .map(str::as_bytes)
            .array_chunks::<3>()
            .map(|chunk| {
                let dup = chunk[0]
                    .iter()
                    .find(|x| chunk[1].contains(x) & chunk[2].contains(x))
                    .unwrap();
                u32::from((dup - 38) % 58)
            })
            .sum::<u32>()
            .into()
    }
}

#[cfg(test)]
bench! {2022, 3, DayThree, year_2022, Solution::U32(7824), Solution::U32(2798)}

#[cfg(test)]
mod tests {
    #[test]
    fn map_char_to_value() {
        let a = b'a';
        let cap_a = b'A';
        assert_eq!(1, (a - 38) % 58);
        assert_eq!(27, (cap_a - 38) % 58);
    }
}
