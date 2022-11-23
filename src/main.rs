use advent_of_code::*;
use std::{env, fs, time::Instant};

fn help() {
    println!(
        "Usage:
day: <int>
    What day to run
part: <int>
    What part to run
"
    );
}

fn main() -> Result<()> {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    let (day, part) = match args.len() {
        3 => {
            let day_raw = &args[1];
            let part_raw = &args[2];
            let day: u32 = day_raw.parse()?;
            let part: u32 = part_raw.parse()?;
            Ok((day, part))
        }
        _ => {
            help();
            Err(Error::ArgParseError("Wrong number of arguments".to_owned()))
        }
    }?;

    let content = fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap();
    let solver: Box<dyn AdventSolver> = match day {
        0 => Box::new(DayZero {}),
        2 => Box::new(DayTwo {}),
        _ => unimplemented!("Day {day} is unimplemented"),
    };

    let solved = match part {
        1 => Some(solver.part_one(&content)),
        2 => Some(solver.part_two(&content)),
        _ => {
            print!("{part} is not a valid part");
            None
        }
    };
    if let Some(solved) = solved {
        println!(
            "Solved day {day} - part {part} in {:?}\n",
            (start_time - Instant::now()).as_nanos()
        );
        println!("Answer:\n{solved}");
    }

    Ok(())
}
