use std::str::FromStr;

pub fn run() {
    let result: usize = include_str!("input")
        .lines()
        .flat_map(str::parse::<Hands>)
        .map(|h| pick_score(&h.mine) + game_score(&h.opponent, &h.mine))
        .sum();
    println!("Part 1: {}", result);

    let result: usize = include_str!("input")
        .lines()
        .flat_map(str::parse::<HandAndOutcome>)
        .map(|ho| {
            let my_pick = outcome_to_pick(&ho.outcome, &ho.opponent);
            return Hands {
                opponent: ho.opponent,
                mine: my_pick,
            };
        })
        .map(|h| pick_score(&h.mine) + game_score(&h.opponent, &h.mine))
        .sum();
    println!("Part 2: {}", result);
}

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum OUTCOME {
    Lose,
    Draw,
    Win,
}

struct Hands {
    mine: RPS,
    opponent: RPS,
}

impl FromStr for Hands {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let hands = match s.split_once(" ") {
            Some((oppo, my)) => Hands {
                opponent: oppo_pick(oppo),
                mine: my_pick(my),
            },
            None => return Err(anyhow::anyhow!("Invalid input!")),
        };

        return Ok(hands);
    }
}

struct HandAndOutcome {
    opponent: RPS,
    outcome: OUTCOME,
}

impl FromStr for HandAndOutcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let hand_outcome = match s.split_once(" ") {
            Some((oppo, my)) => HandAndOutcome {
                opponent: oppo_pick(oppo),
                outcome: letter_to_outcome(my),
            },
            None => return Err(anyhow::anyhow!("Invalid input!")),
        };

        return Ok(hand_outcome);
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

fn oppo_pick(letter: &str) -> RPS {
    match letter {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => unreachable!("This is an unreachable branch!"),
    }
}

fn my_pick(letter: &str) -> RPS {
    match letter {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => unreachable!("This is an unreachable branch!"),
    }
}

fn letter_to_outcome(letter: &str) -> OUTCOME {
    match letter {
        "X" => OUTCOME::Lose,
        "Y" => OUTCOME::Draw,
        "Z" => OUTCOME::Win,
        _ => unreachable!("This is an unreachable branch!"),
    }
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
