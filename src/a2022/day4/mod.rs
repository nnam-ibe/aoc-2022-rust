use std::str::FromStr;

pub fn run() {
    let res = include_str!("input")
        .lines()
        .flat_map(str::parse::<Pair>)
        .filter(|h| {
            return ((h.p1_l <= h.p2_l) && (h.p1_h >= h.p2_h))
                || ((h.p2_l <= h.p1_l) && (h.p2_h >= h.p1_h));
        })
        .count();

    println!("Part 1: {}", res);

    let res = include_str!("input")
        .lines()
        .flat_map(str::parse::<Pair>)
        .filter(|h| {
            return (h.p1_h >= h.p2_l) && (h.p1_l <= h.p2_h);
        })
        .count();

    println!("Part 2: {}", res);
}

struct Pair {
    p1_l: usize,
    p1_h: usize,
    p2_l: usize,
    p2_h: usize,
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (r1, r2) = match s.split_once(",") {
            None => return Err(anyhow::anyhow!("Invalid Input!")),
            Some(val) => val,
        };

        let (p1_l, p1_h) = parse_range(&r1);
        let (p2_l, p2_h) = parse_range(&r2);

        return Ok(Pair {
            p1_l,
            p1_h,
            p2_l,
            p2_h,
        });
    }
}

fn parse_range(s: &str) -> (usize, usize) {
    let res: (usize, usize) = match s.split_once("-") {
        None => unreachable!("Invalid Input"),
        Some((low, high)) => (
            match low.parse() {
                Err(_) => unreachable!("Error parsing number"),
                Ok(n) => n,
            },
            match high.parse() {
                Err(_) => unreachable!("Error parsing number"),
                Ok(n) => n,
            },
        ),
    };
    return res;
}
