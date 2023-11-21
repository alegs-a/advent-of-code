use std::fs::read_to_string;

mod day;
mod year_2022;

// The below is shamelessly stolen from bpaul
fn run_part1(file: &str, day: &day::Day) -> std::io::Result<String> {
    let file = read_to_string(file)?;
    Ok((day.part1)(file))
}

fn run_part2(file: &str, day: &day::Day) -> std::io::Result<String> {
    let file = read_to_string(file)?;
    Ok((day.part2)(file))
}

macro_rules! run_year {
    ($year:ident, $year_num:expr) => {
        for (i, day) in $year::DAYS.iter().enumerate() {
            let input_file = format!("input/{}/{}.txt", $year_num, i + 1);

            println!("Day {} part 1: {}", i + 1, run_part1(&input_file, day)?);
            println!("Day {} part 2: {}", i + 1, run_part2(&input_file, day)?);
        }
    };
}

fn main() -> std::io::Result<()> {
    // let year = 2022;
    // let args = std::env::args().collect::<Vec<String>>();
    // let day = args.get(1).unwrap().parse::<i32>().unwrap();

    // let file = read_to_string(format!("input/{}/{}.txt", year, day));
    run_year!(year_2022, 2022);

    Ok(())
}
