use crate::aoc;

pub async fn solve() {
    let input = aoc::read_input_or_fetch(2).await.unwrap();

    let frequencies = input
        .lines()
        .map(|line| {
            line.chars()
                .fold(std::collections::HashMap::new(), |mut acc, c| {
                    *acc.entry(c).or_insert(0) += 1;
                    acc
                })
        })
        .collect::<Vec<_>>();

    let twos = frequencies
        .iter()
        .filter(|line| line.values().any(|&v| v == 2))
        .count();

    let threes = frequencies
        .iter()
        .filter(|line| line.values().any(|&v| v == 3))
        .count();

    println!("Day 2. Part 1: {}", twos * threes);

    let lines = input.lines().collect::<Vec<_>>();

    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let left = lines[i];
            let right = lines[j];

            if left.len() != right.len() {
                continue;
            }

            let mut diff = 0;
            let mut pos = 0;
            let mut last_diff = 0;

            for (l, r) in left.chars().zip(right.chars()) {
                if l != r {
                    diff += 1;
                    last_diff = pos;
                }
                pos += 1;
            }

            if diff == 1 {
                let result = left[0..last_diff].to_string() + &left[last_diff + 1..];
                println!("Day 2. Part 2: {}", result);
                return;
            }
        }
    }
}
