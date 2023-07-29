mod day1;
mod day2;
mod day3;

use super::Days;

pub fn run(d: Days) {
    match d {
        Days::Day1 => day1::run(),
        Days::Day2 => day2::run(),
        Days::Day3 => day3::run(),
    }
}
