use std::collections::HashSet;

fn get_code(val: &char) -> usize {
    let code = *val as usize;
    if code >= 65 && code <= 90 {
        return code - 65 + 27;
    } else if code >= 97 && code <= 122 {
        return code - 97 + 1;
    }
    unreachable!("Please specify a valid char");
}

pub fn run() {
    let sum: usize = include_str!("input")
        .lines()
        .map(|l| {
            let len = l.len();
            return l.split_at(len / 2);
        })
        .map(|(sac1, sac2)| {
            let set1: HashSet<char> = sac1.chars().collect();
            return (set1, sac2);
        })
        .flat_map(|(set1, sac2)| {
            return sac2.chars().find(|val| set1.contains(&val));
        })
        .map(|dup_char| get_code(&dup_char))
        .sum();

    println!("Part 1: {}", sum);

    // PART 2
    let lines: Vec<&str> = include_str!("input").lines().collect();
    let priorities: usize = lines
        .chunks(3)
        .map(|chunk| {
            let mut first: HashSet<char> = chunk[0].chars().collect();
            let second: HashSet<char> = chunk[1].chars().collect();
            let third: HashSet<char> = chunk[2].chars().collect();

            first.clone().into_iter().for_each(|f| {
                if !second.contains(&f) {
                    first.remove(&f);
                } else if !third.contains(&f) {
                    first.remove(&f);
                }
            });

            let res = first.into_iter().take(1).next();
            return match res {
                None => unreachable!("There should be at least one match"),
                Some(val) => val,
            };
        })
        .map(|x| get_code(&x))
        .sum();

    println!("Part 2: {}", priorities);
}
