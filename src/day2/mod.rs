use super::utils::file_to_string;
use std::fmt::Display;

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl Display for RPS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            RPS::Rock => "Rock",
            RPS::Paper => "Paper",
            RPS::Scissors => "Scissors",
        };

        return write!(f, "{}", t);
    }
}

enum OUTCOME {
    Lose,
    Draw,
    Win,
}

impl Display for OUTCOME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            OUTCOME::Lose => "Lose",
            OUTCOME::Draw => "Draw",
            OUTCOME::Win => "win",
        };

        return write!(f, "{}", t);
    }
}

fn game_score(oppo: &RPS, my: &RPS) -> usize {
    match my {
        RPS::Rock => match oppo {
            RPS::Rock => return 3,
            RPS::Paper => return 0,
            RPS::Scissors => return 6,
        },
        RPS::Paper => match oppo {
            RPS::Rock => return 6,
            RPS::Paper => return 3,
            RPS::Scissors => return 0,
        },
        RPS::Scissors => match oppo {
            RPS::Rock => return 0,
            RPS::Paper => return 6,
            RPS::Scissors => return 3,
        },
    }
}

fn pick_score(choice: &RPS) -> usize {
    match choice {
        RPS::Rock => return 1,
        RPS::Paper => return 2,
        RPS::Scissors => return 3,
    }
}

fn oppo_pick(letter: char) -> RPS {
    match letter {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => unreachable!("This is an unreachable branch!"),
    }
}

fn my_pick(letter: char) -> RPS {
    match letter {
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors,
        _ => unreachable!("This is an unreachable branch!"),
    }
}

fn letter_to_outcome(letter: char) -> OUTCOME {
    match letter {
        'X' => OUTCOME::Lose,
        'Y' => OUTCOME::Draw,
        'Z' => OUTCOME::Win,
        _ => unreachable!("This is an unreachable branch!"),
    }
}

pub fn run() {
    let file = file_to_string("src/day2/input");
    let result: usize = file
        .lines()
        .map(|x| {
            let mut chars = x.chars();
            let raw_oppo_move = chars.next();
            let oppo_move = oppo_pick(raw_oppo_move.unwrap());

            chars.next();

            let raw_my_move = chars.next();
            let my_move = my_pick(raw_my_move.unwrap());

            return pick_score(&my_move) + game_score(&oppo_move, &my_move);
        })
        .sum();

    println!("{}", result);
}

fn outcome_to_pick(outcome: &OUTCOME, oppo: &RPS) -> RPS {
    match outcome {
        OUTCOME::Lose => match oppo {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        },
        OUTCOME::Draw => match oppo {
            RPS::Rock => RPS::Rock,
            RPS::Paper => RPS::Paper,
            RPS::Scissors => RPS::Scissors,
        },
        OUTCOME::Win => match oppo {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        },
    }
}

pub fn run2() {
    let file = file_to_string("src/day2/input");
    let result: usize = file
        .lines()
        .map(|x| {
            let mut chars = x.chars();
            let raw_oppo_move = chars.next();
            let oppo_move = oppo_pick(raw_oppo_move.unwrap());

            chars.next();

            let raw_my_move = chars.next();
            let my_outcome = letter_to_outcome(raw_my_move.unwrap());
            let my_move = outcome_to_pick(&my_outcome, &oppo_move);

            return pick_score(&my_move) + game_score(&oppo_move, &my_move);
        })
        .sum();

    println!("{}", result);
}
