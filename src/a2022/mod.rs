mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use super::Days;

pub fn run(d: Days) {
    match d {
        Days::Day1 => day1::run(),
        Days::Day2 => day2::run(),
        Days::Day3 => day3::run(),
        Days::Day4 => day4::run(),
        Days::Day5 => day5::run(),
        Days::Day6 => day6::run(),
    }
}
