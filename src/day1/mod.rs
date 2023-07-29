use std::fmt::Display;

pub fn top() {
    let file = std::fs::read_to_string("src/day1/input").expect("File should exist");
    let mut max: usize = 0;
    let mut index: usize = 0;

    file.rsplit("\n\n").enumerate().for_each(|(idx, group)| {
        let total: usize = group.lines().map(|l| l.parse::<usize>().unwrap_or(0)).sum();

        if total > max {
            max = total;
            index = idx;
        }
    });

    println!("max {} at index {}", max, index);
}

pub fn top_three() {
    let mut q = PriorityQ::default();

    let file = std::fs::read_to_string("src/day1/input").expect("File should exist");

    file.rsplit("\n\n").for_each(|group| {
        let total: usize = group.lines().map(|l| l.parse::<usize>().unwrap_or(0)).sum();

        q.push(total);
    });

    println!("State {}", q);
    println!("Top Three Sum => {}", q.data[0] + q.data[1] + q.data[2]);
}

struct PriorityQ {
    data: Vec<usize>,
}

impl PriorityQ {
    fn push(&mut self, item: usize) {
        if self.data.len() < 3 {
            self.data.push(item);
        } else {
            let val = self.data.get(2).unwrap();
            if item > *val {
                self.data.remove(2);
                self.data.push(item)
            }
        }

        self.data.sort_by(|a, b| b.cmp(a))
    }
}

impl Default for PriorityQ {
    fn default() -> Self {
        return PriorityQ { data: vec![] };
    }
}

impl Display for PriorityQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default: usize = 0;
        return write!(
            f,
            "Queue[{}, {}, {}]:",
            self.data.get(0).unwrap_or(&default),
            self.data.get(1).unwrap_or(&default),
            self.data.get(2).unwrap_or(&default),
        );
    }
}
