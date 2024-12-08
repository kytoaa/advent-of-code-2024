#[allow(dead_code)]
pub fn run(mut dir: String) {
    dir.push_str("/day7.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data);
    println!("{}", result);
}

fn solve(data: &str) -> u64 {
    let equations = parse_equations(data);

    equations
        .into_iter()
        .filter_map(|mut e| e.evaluate(0))
        .sum()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Add(u64),
    Mul(u64),
    Concat(u64),
}
impl Operator {
    fn value(&self) -> u64 {
        match self {
            Operator::Add(n) => *n,
            Operator::Mul(n) => *n,
            Operator::Concat(n) => *n,
        }
    }
    fn rotate(&mut self) {
        match &self {
            Operator::Add(n) => *self = Operator::Mul(*n),
            Operator::Mul(n) => *self = Operator::Concat(*n),
            Operator::Concat(n) => *self = Operator::Add(*n),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Equation {
    result: u64,
    numbers: Vec<Operator>,
}
impl Equation {
    fn is_valid(&self) -> bool {
        let mut n = self.numbers.first().unwrap().value();

        for o in &self.numbers[1..] {
            match o {
                Operator::Add(v) => n += v,
                Operator::Mul(v) => n *= v,
                Operator::Concat(v) => {
                    let mut n_s = n.to_string();
                    n_s.push_str(&v.to_string());
                    n = n_s.parse().unwrap();
                }
            }
        }

        n == self.result
    }
    fn evaluate(&mut self, i: usize) -> Option<u64> {
        for _ in 0..3 {
            if self.is_valid() {
                return Some(self.result);
            }
            if i < (self.numbers.len() - 1) {
                if let Some(n) = self.evaluate(i + 1) {
                    return Some(n);
                }
            }
            self.numbers.get_mut(i).unwrap().rotate();
        }

        None
    }
}

fn parse_equations(data: &str) -> Vec<Equation> {
    data.trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let numbers: Vec<&str> = line.split(' ').collect();
            let result = numbers[0][..numbers[0].len() - 1].parse().unwrap();

            Equation {
                result,
                numbers: numbers[1..]
                    .into_iter()
                    .map(|n| Operator::Add(n.parse().unwrap()))
                    .collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn parse_test() {
        let shorter_example = "3267: 81 40 27
83: 17 5
156: 15 6";

        use Operator as Op;
        assert_eq!(
            parse_equations(shorter_example),
            vec![
                Equation {
                    result: 3267,
                    numbers: vec![Op::Add(81), Op::Add(40), Op::Add(27)]
                },
                Equation {
                    result: 83,
                    numbers: vec![Op::Add(17), Op::Add(5)]
                },
                Equation {
                    result: 156,
                    numbers: vec![Op::Add(15), Op::Add(6)]
                }
            ]
        );
    }

    #[test]
    fn example_test() {
        assert_eq!(solve(EXAMPLE), 11387);
    }
}
