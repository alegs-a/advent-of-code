use std::env;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_10;

use day_1::day_one;
use day_2::day_two;
use day_3::day_three;
use day_4::day_four;
use day_5::day_five;
use day_6::day_six;
use day_7::day_seven;
use day_8::day_eight;
use day_9::day_nine;
use day_10::day_ten;

fn main() {
    let args: Vec<String> = env::args().collect();
    let first_arg = match args.get(1) {
        Some(val) => val,
        None => return,
    };
    let day: i32 = match first_arg.parse() {
        Ok(val) => val,
        _ => return,
    };

    match day {
        1 => day_one(),
        2 => day_two(),
        3 => day_three(),
        4 => day_four(),
        5 => day_five(),
        6 => day_six(),
        7 => day_seven(),
        8 => day_eight(),
        9 => day_nine(),
        10 => day_ten(),
        _ => {
            println!("Uh oh!, I haven't written any code for that day yet :(")
        }
    }
}
