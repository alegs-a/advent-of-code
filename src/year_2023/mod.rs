use crate::day::Day;
use crate::make_day;

pub const DAYS_NUM: usize = 21;

pub const DAYS: [Day; DAYS_NUM] = [
    make_day!(day_1),
    make_day!(day_2),
    make_day!(day_3),
    make_day!(day_4),
    make_day!(day_5),
    make_day!(day_6),
    make_day!(day_7),
    make_day!(day_8),
    make_day!(day_9),
    make_day!(day_10),
    make_day!(day_11),
    make_day!(day_12),
    make_day!(day_13),
    make_day!(day_14),
    make_day!(day_15),
    make_day!(day_16),
    make_day!(day_17),
    make_day!(day_18),
    make_day!(day_19),
    make_day!(day_20),
    make_day!(day_21),
];

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
#[allow(warnings)]
mod day_19;
mod day_20;
mod day_21;
