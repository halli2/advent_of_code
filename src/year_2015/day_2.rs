use crate::{AdventSolver, Solution};

pub struct DayTwo {}

impl AdventSolver for DayTwo {
    fn part_one(&self, input: &str) -> Solution {
        let res = input
            .lines()
            .map(|l| {
                let mut xyz = l.split('x');
                let x: i32 = xyz.next().unwrap().parse().unwrap();
                let y: i32 = xyz.next().unwrap().parse().unwrap();
                let z: i32 = xyz.next().unwrap().parse().unwrap();
                let slack = (x * y * z) / x.max(y.max(z));
                2 * (x * y + y * z + x * z) + slack
            })
            .sum::<i32>();
        res.into()
    }

    fn part_two(&self, input: &str) -> Solution {
        let res = input
            .lines()
            .map(|l| {
                let mut xyz = l.split('x');
                let l: i32 = xyz.next().unwrap().parse().unwrap();
                let w: i32 = xyz.next().unwrap().parse().unwrap();
                let h: i32 = xyz.next().unwrap().parse().unwrap();
                let perimeter = 2 * (l + w).min((w + h).min(l + h));
                let ribbon = l * h * w;
                perimeter + ribbon
            })
            .sum::<i32>();
        res.into()
    }
}
