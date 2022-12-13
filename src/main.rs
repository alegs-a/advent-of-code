use std::env;

pub mod day_1;
pub mod day_2;
pub mod day_3;

use day_1::day_one;
use day_2::day_two;
use day_3::day_three;

fn main() {
    let args: Vec<String> = env::args().collect();
    let first_arg = match args.get(1) {
        Some(val) => val,
        None => return
    };
    let day: i32 = match first_arg.parse() {
        Ok(val) => val,
        _ => return
    };

    match day {
        1 => day_one(),
        2 => day_two(),
        3 => day_three(),
        _ => {
            println!("Uh oh!, I haven't written any code for that day yet :(")
        }
    }
}
