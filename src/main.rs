use advent_of_code::*;
use color_eyre::eyre::eyre;
use std::{env, fs, time::Instant};

static CURRENT_YEAR: u32 = 2022;

fn help() {
    println!(
        "Usage:
day: <int>
    What day to run
part: <int>
    What part to run (part 1, part 2, and sometimes part 3 for a visualizer)
"
    );
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = env::args().collect();
    let (day, part, year) = if let 3 | 4 = args.len() {
        let day_raw = &args[1];
        let part_raw = &args[2];
        let day: u32 = day_raw.parse()?;
        let part: u32 = part_raw.parse()?;
        let year = match args.get(3) {
            Some(v) => v.parse()?,
            None => CURRENT_YEAR,
        };
        Ok((day, part, year))
    } else {
        help();
        Err(eyre!("Wrong number of arguments"))
    }?;

    let content = fs::read_to_string(format!("./data/{}/day{:0>2}.txt", year, day))?;
    let solver: Box<dyn AdventSolver> = match year {
        2015 => year_2015::get_day(day),
        2022 => year_2022::get_day(day),
        _ => unimplemented!("Year {year} not done"),
    };

    let start_time = Instant::now();
    let solved = match part {
        1 => Some(solver.part_one(&content)),
        2 => Some(solver.part_two(&content)),
        3 => {
            solver.visualize(&content);
            return Ok(());
        }
        _ => {
            print!("{part} is not a valid part");
            None
        }
    };
    let elapsed = start_time.elapsed().as_nanos() as f64 / 1_000.0;
    if let Some(solved) = solved {
        println!("Solved day {day} - part {part} in {:.2} µs", elapsed);
        println!("Answer:\n{solved}");
    }

    Ok(())
}
