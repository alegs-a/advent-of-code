#![allow(warnings)]
use crate::day::Day;
use crate::make_day;

pub const DAYS_NUM: usize = 11;

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
];

mod day_1;
mod day_10;
mod day_11;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
