use crate::aoc;

pub async fn solve() {
    let input = aoc::read_input_or_fetch(1).await.unwrap();

    let lines = input
        .lines()
        .map(|line| line.split_at(1))
        .map(|(op, num)| match op {
            "+" => num.parse::<i32>().unwrap(),
            "-" => -num.parse::<i32>().unwrap(),
            _ => 0,
        })
        .collect::<Box<[i32]>>();

    let frequencies = (0..).scan(0, |state, idx| {
        *state += lines[idx as usize % lines.len()];
        Some(*state)
    });

    println!(
        "Day 1. Part 1: {}",
        frequencies.clone().nth(lines.len() - 1).unwrap()
    );

    let mut seen = std::collections::HashSet::new();
    'outer: loop {
        for freq in frequencies.clone() {
            if !seen.insert(freq) {
                println!("Day 1. Part 2: {}", freq);
                break 'outer;
            }
        }
    }
}
