use std::collections::HashMap;

pub fn run(mut dir: String) {
    dir.push_str("/day_11.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data);
    println!("{result}");
}

fn solve(data: &str) -> usize {
    let stones = parse_stones(data);

    let mut stone_counts = HashMap::new();
    for stone in stones {
        stone_counts.insert(stone, 1);
    }

    for _ in 0..75 {
        let mut new_stone_counts = HashMap::new();

        for (stone, count) in stone_counts
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(Stone, usize)>>()
        {
            let digits = stone.to_string();

            for n in if stone == 0 {
                vec![1].into_iter()
            } else if digits.len() % 2 == 0 {
                vec![
                    digits[..(digits.len() / 2)].parse::<Stone>().unwrap(),
                    digits[(digits.len() / 2)..].parse::<Stone>().unwrap(),
                ]
                .into_iter()
            } else {
                vec![stone * 2024].into_iter()
            } {
                match new_stone_counts.get_mut(&n) {
                    Some(c) => *c += count,
                    None => _ = new_stone_counts.insert(n, count),
                }
            }
        }
        stone_counts = new_stone_counts;
    }

    stone_counts.values().sum()
}

type Stone = u64;

fn parse_stones(data: &str) -> Vec<Stone> {
    data.trim()
        .split_whitespace()
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}
