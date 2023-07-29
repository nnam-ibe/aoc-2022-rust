mod day1;
mod day2;

use super::Days;

pub fn run(d: Days) {
    match d {
        Days::Day1 => day1::run(),
        Days::Day2 => day2::run(),
    }
}
