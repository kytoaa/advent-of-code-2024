pub fn run(mut dir: String) {
    dir.push_str("/day_11.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data);
    println!("{result}");
}

fn solve(data: &str) -> usize {
    let mut stones = parse_stones(data);

    for _ in 0..25 {
        stones = step(stones);
    }

    stones.len()
}

type Stone = u64;

fn step(stones: Vec<Stone>) -> Vec<Stone> {
    stones
        .into_iter()
        .map(|stone| {
            let digits = stone.to_string();
            if digits.chars().next().unwrap() == '0' {
                vec![digits
                    .chars()
                    .map(|c| match c {
                        '0' => '1',
                        c => c,
                    })
                    .collect::<String>()
                    .parse::<Stone>()
                    .unwrap()]
                .into_iter()
            } else if digits.len() % 2 == 0 {
                vec![
                    digits[..(digits.len() / 2)].parse::<Stone>().unwrap(),
                    digits[(digits.len() / 2)..].parse::<Stone>().unwrap(),
                ]
                .into_iter()
            } else {
                vec![stone * 2024].into_iter()
            }
        })
        .flatten()
        .collect()
}

fn parse_stones(data: &str) -> Vec<Stone> {
    data.trim()
        .split_whitespace()
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_test() {
        let stones = vec![0, 1, 10, 99, 999];

        assert_eq!(step(stones), vec![1, 2024, 1, 0, 9, 9, 2021976])
    }

    #[test]
    fn example_test() {
        let example = "125 17";

        assert_eq!(solve(example), 55312);
    }
}
