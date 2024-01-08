mod aoc;

#[tokio::main]
async fn main() {
    let day_parse_result = std::env::args().nth(1).map(|s| s.parse::<u8>().unwrap());

    let day_number = match day_parse_result {
        Some(day) => day,
        None => {
            println!("Day is not specified. Usage: `cargo run <day>`");
            return;
        }
    };

    match day_number {
        1 => solutions::day1().await,
        2 => solutions::day2().await,
        3 => solutions::day3().await,
        _ => panic!("Unknown day: {}", day_number),
    }
}

pub mod solutions {
    use crate::{aoc, IterExt};

    pub async fn day1() {
        let input = aoc::read_input_or_fetch(1).await.unwrap();

        let lines = input
            .lines()
            .map(|line| line.split_at(1))
            .map(|(op, num)| match op {
                "+" => num.parse::<i32>().unwrap(),
                "-" => -num.parse::<i32>().unwrap(),
                _ => panic!("Unknown operator: {}", op),
            })
            .collect::<Vec<_>>();

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

    pub async fn day2() {
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

        let result = input
            .lines()
            .collect::<Vec<_>>()
            .combinations(2)
            .iter()
            .map(|pair| pair[0].chars().zip(pair[1].chars()))
            .find(|line| line.clone().filter(|(l, r)| l != r).count() == 1)
            .unwrap()
            .filter(|(l, r)| l == r)
            .map(|(l, _)| l)
            .collect::<String>();

        println!("Day 2. Part 2: {}", result);
    }

    pub async fn day3() {
        let input = aoc::read_input_or_fetch(3).await.unwrap();
    }
}

trait IterExt<T> {
    fn combinations(&self, r: usize) -> Vec<Vec<&T>>;
}

impl<T> IterExt<T> for [T]
where
    T: Clone,
{
    fn combinations(&self, r: usize) -> Vec<Vec<&T>> {
        if r == 0 {
            return vec![vec![]];
        }

        if r > self.len() {
            return vec![];
        }

        if r == self.len() {
            return vec![self.iter().collect()];
        }

        if r == 1 {
            return self.iter().map(|item| vec![item]).collect();
        }

        let mut result = Vec::new();
        for (i, item) in self.iter().enumerate() {
            let rest_combinations = self[(i + 1)..].combinations(r - 1);
            for combination in rest_combinations {
                let mut combo = vec![item];
                combo.extend_from_slice(&combination);
                result.push(combo);
            }
        }

        result
    }
}
