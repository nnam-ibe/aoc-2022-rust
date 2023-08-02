use std::collections::HashMap;

pub fn run() {
    let marker = 14;
    let mut map: HashMap<char, usize> = HashMap::new();
    include_str!("input").lines().for_each(|line| {
        let mut idx = 0;
        let mut counter = 0;
        while idx < line.len() && counter < marker {
            match line.chars().nth(idx) {
                None => unreachable!("No way no"),
                Some(c) => {
                    if let Some(prev) = map.get(&c) {
                        let diff = idx - prev;
                        if diff <= counter {
                            counter = diff;
                        } else {
                            counter += 1;
                        }
                    } else {
                        counter += 1;
                    }
                    map.insert(c, idx);
                }
            }
            idx += 1;
        }
        println!("{}", idx);
        map = HashMap::new();
    });
}
