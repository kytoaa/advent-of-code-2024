#[allow(dead_code)]
pub fn run(mut dir: String) {
    dir.push_str("/day_13.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data);
    println!("{result} tokens");
}

fn solve(data: &str) -> u64 {
    let claw_machines = parse_claw_machines(data);

    claw_machines
        .into_iter()
        .filter_map(|machine| machine.solve())
        .sum()
}

#[derive(Debug, PartialEq)]
struct ClawMachine {
    a: Vector,
    b: Vector,
    prize: Vector,
}

impl ClawMachine {
    fn solve(&self) -> Option<u64> {
        let a = ((self.b.x * self.prize.y) - (self.b.y * self.prize.x))
            / ((self.a.y * self.b.x) - (self.a.x * self.b.y));

        let b = (self.prize.x - (self.a.x * a)) / self.b.x;

        if (self.a.x * a) + (self.b.x * b) - self.prize.x != 0
            || (self.a.y * a) + (self.b.y * b) - self.prize.y != 0
        {
            return None;
        }

        Some(((3 * a) + b) as u64)
    }
}

fn parse_claw_machines(data: &str) -> Vec<ClawMachine> {
    let lines: Vec<&str> = data
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    let mut claw_machines = vec![];

    for i in 0..(lines.len() / 3) {
        let i = i * 3;

        let button_a = {
            let button_a = &lines[i].trim()[12..];
            let (x, len) = parse_number(button_a);
            let button_a = &button_a[(len + 4)..];
            let (y, _) = parse_number(button_a);
            (x, y)
        };

        let button_b = {
            let button_b = &lines[i + 1].trim()[12..];
            let (x, len) = parse_number(button_b);
            let button_b = &button_b[(len + 4)..];
            let (y, _) = parse_number(button_b);
            (x, y)
        };

        let coords = {
            let coords = &lines[i + 2].trim()[9..];
            let (x, len) = parse_number(coords);
            let coords = &coords[(len + 4)..];
            let (y, _) = parse_number(coords);
            (x, y)
        };

        claw_machines.push(ClawMachine {
            a: Vector::new(button_a.0, button_a.1),
            b: Vector::new(button_b.0, button_b.1),
            prize: Vector::new(coords.0 + 10000000000000, coords.1 + 10000000000000),
        });
    }

    claw_machines
}
fn parse_number(number: &str) -> (i64, usize) {
    let mut i = 0;
    while let Some(c) = number.chars().skip(i).next() {
        if c.is_digit(10) {
            i += 1;
        } else {
            break;
        }
    }
    (number[0..(i)].trim().parse().unwrap(), i)
}

#[derive(Debug, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
}
impl Vector {
    fn new(x: i64, y: i64) -> Vector {
        Vector { x, y }
    }
}
impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl std::ops::Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_claw_machine_tests() {
        let machines = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";

        assert_eq!(
            parse_claw_machines(machines),
            vec![
                ClawMachine {
                    a: Vector::new(94, 34),
                    b: Vector::new(22, 67),
                    prize: Vector::new(10000000008400, 10000000005400),
                },
                ClawMachine {
                    a: Vector::new(26, 66),
                    b: Vector::new(67, 21),
                    prize: Vector::new(10000000012748, 10000000012176),
                },
            ]
        );
    }
}
