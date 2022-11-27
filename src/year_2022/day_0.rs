use crate::AdventSolver;
use chumsky::prelude::*;
use rayon::prelude::*;

#[derive(Debug)]
pub enum Cmd {
    Forward(i32),
    Up(i32),
    Down(i32),
}

// impl FromStr for Cmd {
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut input = s.split_whitespace();
//         let dir = input.next().unwrap();
//         let count = input.next().unwrap().parse().unwrap();
//         match dir {
//             "forward" => Ok(Cmd::Forward(count)),
//             "up" => Ok(Cmd::Up(-count)),
//             "down" => Ok(Cmd::Down(-count)),
//             _ => Err("ouch".to_owned()),
//         }
//     }
// }

pub fn example_parser() -> impl Parser<char, Vec<Cmd>, Error = Simple<char>> {
    let int = text::int(10).map(|s: String| s.parse().unwrap());

    let cmd_line = text::ident()
        .then_ignore(just(' '))
        .then(int)
        .map(|(cmd, count)| match cmd.as_str() {
            "forward" => Cmd::Forward(count),
            "up" => Cmd::Up(count),
            "down" => Cmd::Down(count),
            _ => panic!(),
        });
    cmd_line.separated_by(text::newline())
}

/// Example solver
pub struct DayZero {}

impl AdventSolver for DayZero {
    fn part_one(&self, input: &str) -> String {
        match example_parser().parse(input) {
            Ok(commands) => {
                let (forw, up) = commands
                    .par_iter()
                    .fold(
                        || (0, 0),
                        |(forw, depth), cmd| match cmd {
                            Cmd::Forward(c) => (forw + c, depth),
                            Cmd::Up(c) => (forw, depth - c),
                            Cmd::Down(c) => (forw, depth + c),
                        },
                    )
                    .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
                (forw * up).to_string()
            }
            Err(parse_err) => parse_err.into_iter().map(|c| c.to_string()).collect(),
        }
    }

    /// Example without tools
    fn part_two(&self, input: &str) -> String {
        let (forw, depth) = input.lines().map(|l| l.split_once(" ").unwrap()).fold(
            (0, 0),
            |(forw, depth), (dir, count)| match (dir, count.parse::<i32>().unwrap_or(0)) {
                ("forward", v) => (forw + v, depth),
                ("down", v) => (forw, depth + v),
                ("up", v) => (forw, depth - v),
                _ => panic!(),
            },
        );
        (forw * depth).to_string()
    }
}
