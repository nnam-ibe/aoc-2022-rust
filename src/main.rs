mod a2022;

use a2022::run;

pub enum Days {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let val = match args.get(1) {
        None => unreachable!("Specify a reachable day!"),
        Some(val) => val,
    };

    match val.as_str() {
        "1" => run(Days::Day1),
        "2" => run(Days::Day2),
        "3" => run(Days::Day3),
        "4" => run(Days::Day4),
        "5" => run(Days::Day5),
        _ => unreachable!("Day not done yet"),
    }
}
