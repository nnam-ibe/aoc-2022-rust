use regex::Regex;
use std::str::FromStr;

pub fn run() {
    let regex = match Regex::new(r"\[(\w)\]") {
        Err(_) => unreachable!("Be that as it may"),
        Ok(val) => val,
    };

    let mut stacks: Vec<Vec<String>> = vec![];
    let (formation, instructions) = match include_str!("input").split_once("\n\n") {
        None => unreachable!("Invalid Input!"),
        Some(val) => val,
    };

    formation
        .lines()
        .take_while(|line| line.contains("["))
        .for_each(|line| {
            line.as_bytes()
                .chunks(4)
                .flat_map(std::str::from_utf8)
                .enumerate()
                .for_each(|(idx, data)| {
                    if stacks.len() <= idx {
                        stacks.push(vec![]);
                    }

                    if let Some(captures) = regex.captures(data) {
                        if let Some(first) = captures.get(1) {
                            stacks[idx].push(String::from(first.as_str()));
                        }
                    }
                })
        });

    stacks.iter_mut().for_each(|v| v.reverse());
    let mut stacks2: Vec<Vec<String>> = stacks.clone();

    // Part One
    instructions
        .clone()
        .lines()
        .flat_map(str::parse::<Move>)
        .for_each(|d| {
            let mut num = d.num;
            while num > 0 {
                if let Some(payload) = stacks[d.from].pop() {
                    stacks[d.to].push(payload);
                }
                num -= 1;
            }
        });

    let mut res: Vec<String> = vec![];
    for mut stack in stacks {
        match stack.pop() {
            None => res.push(String::from(" ")),
            Some(v) => res.push(v),
        }
    }
    println!("Part One: {:?}", res.join(""));

    // Part Two
    instructions
        .lines()
        .flat_map(str::parse::<Move>)
        .for_each(|d| {
            let len = stacks2[d.from].len();
            let slice = stacks2[d.from].split_off(len - d.num);
            stacks2[d.to].extend_from_slice(&slice);
        });

    let mut res: Vec<String> = vec![];
    for mut stack in stacks2 {
        match stack.pop() {
            None => res.push(String::from(" ")),
            Some(v) => res.push(v),
        }
    }
    println!("Part Two: {:?}", res.join(""));
}

struct Move {
    num: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let re = match Regex::new(r"move (\d+) from (\d+) to (\d+)") {
            Err(_) => unreachable!("Pear"),
            Ok(val) => val,
        };
        if let Some(captures) = re.captures(s) {
            return Ok(Move {
                num: parse_int(&captures[1]),
                from: parse_int(&captures[2]) - 1,
                to: parse_int(&captures[3]) - 1,
            });
        }

        return Err(anyhow::anyhow!("Invalid input!"));
    }
}

fn parse_int(str: &str) -> usize {
    match str.parse::<usize>() {
        Err(_) => unreachable!("This is an unreachable"),
        Ok(v) => v,
    }
}
