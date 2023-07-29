pub fn run() {
    let mut sums: Vec<usize> = include_str!("input")
        .split("\n\n")
        .map(|calories| calories.lines().flat_map(str::parse::<usize>).sum())
        .collect();

    let max = match sums.iter().max() {
        Some(v) => v,
        None => &0,
    };
    println!("Part One: {:?}", max);

    sums.sort_by(|a, b| b.cmp(a));
    let ans = sums.into_iter().take(3).sum::<usize>();
    println!("Part Two: {:?}", ans);
}
