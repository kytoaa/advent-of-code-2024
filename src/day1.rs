use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(mut dir: String) {
    dir.push_str("/day1.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data[..]);
    println!("{}", result);

    let result = similarity_score(&data[..]);
    println!("{}", result);
}

fn solve(data: &str) -> u32 {
    let pairs = get_pairs(&data[..]);

    pairs.map(|(a, b)| a.abs_diff(b)).sum()
}

fn get_pairs(data: &str) -> impl Iterator<Item = (u32, u32)> {
    let (a, b): (Vec<(usize, u32)>, Vec<(usize, u32)>) = data
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    let mut a: Vec<u32> = a.into_iter().map(|(_, n)| n).collect();
    let mut b: Vec<u32> = b.into_iter().map(|(_, n)| n).collect();
    a.sort();
    b.sort();

    a.into_iter().zip(b.into_iter())
}

fn similarity_score(data: &str) -> u32 {
    let mut scores: HashMap<u32, usize> = HashMap::new();

    let (a, b): (Vec<(usize, u32)>, Vec<(usize, u32)>) = data
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    a.into_iter()
        .map(|(_, n)| match scores.get(&n) {
            Some(v) => *v,
            None => {
                let count = b.iter().filter(|(_, v)| *v == n).count() * n as usize;
                scores.insert(n, count);
                count
            }
        } as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn get_pairs_test() {
        let result: Vec<(u32, u32)> = get_pairs(DATA).collect();
        assert_eq!(result, vec![(1, 3), (2, 3), (3, 3), (3, 4), (3, 5), (4, 9)]);
    }

    #[test]
    fn solve_test() {
        let result = solve(DATA);
        assert_eq!(result, 11);
    }

    #[test]
    fn similarity_test() {
        let result = similarity_score(DATA);
        assert_eq!(result, 31);
    }
}
