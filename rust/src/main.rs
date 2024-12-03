use std::fs::read_to_string;

mod day;
mod year_2022;
mod year_2023;
mod year_2024;
mod util;

// The below is shamelessly stolen from bpaul
fn run_part1(file: &str, day: &day::Day) -> std::io::Result<String> {
    let file = read_to_string(file)?;
    Ok((day.part1)(file))
}

fn run_part2(file: &str, day: &day::Day) -> std::io::Result<String> {
    let file = read_to_string(file)?;
    Ok((day.part2)(file))
}

macro_rules! run_day_in_year {
    ($year:ident, $year_num:expr, $day:expr) => {
        let input_file = format!("../input/{}/{}.txt", $year_num, $day);

        let day = $year::DAYS.get(($day - 1) as usize).unwrap();
        println!("Day {} part 1: {}", $day, run_part1(&input_file, day)?);
        println!("Day {} part 2: {}", $day, run_part2(&input_file, day)?);
    };
}

fn main() -> std::io::Result<()> {
    let year = 2023;
    let args = std::env::args().collect::<Vec<String>>();
    let day = args.get(1).unwrap().parse::<i32>().unwrap();

    run_day_in_year!(year_2023, year, day);

    Ok(())
}
