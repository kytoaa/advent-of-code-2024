pub fn run(mut dir: String) {
    dir.push_str("/day_14.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data, Vector::new(101, 103));
    println!("{result}");

    manual_easter_egg_search(&data, Vector::new(101, 103));
}

fn manual_easter_egg_search(data: &str, space_size: Vector) {
    let robots = parse_robots(data);
    let mut space = Space::new(space_size, robots).unwrap();

    println!("[end] to end, digit to step n times");
    let mut i = 0;

    const DEDUP_MANUAL_CHECK_THRESHOLD: usize = 100;

    // not very efficient solution but i dont care much as im trying to catch up to the current day
    loop {
        let displayed = space.display();
        let mut v: Vec<char> = displayed.chars().collect();
        v.dedup();

        if displayed.len() - v.len() < DEDUP_MANUAL_CHECK_THRESHOLD {
            space.step();
            i += 1;
            continue;
        }

        println!("{}\n{i}", displayed);
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        if buffer.trim() == "end" {
            break;
        }

        for _ in 0..buffer.trim().parse::<usize>().unwrap_or(1) {
            space.step();
            i += 1;
        }
    }
}

fn solve(data: &str, space_size: Vector) -> u64 {
    let robots = parse_robots(data);
    let mut space = Space::new(space_size, robots).unwrap();

    for _ in 0..100 {
        space.step();
    }

    space.safety_factor()
}

struct Space {
    size: Vector,
    robots: Vec<Robot>,
}
impl Space {
    fn new(size: Vector, robots: Vec<Robot>) -> Option<Space> {
        if size.x % 2 == 1 && size.y % 2 == 1 {
            Some(Space { size, robots })
        } else {
            None
        }
    }
    fn step(&mut self) {
        for robot in self.robots.iter_mut() {
            robot.step();
        }
    }
    fn safety_factor(&self) -> u64 {
        let (x_middle, y_middle) = (self.size.x / 2, self.size.y / 2);

        (0..4)
            .map(|i| {
                let n = self
                    .robots
                    .iter()
                    .filter(|robot| {
                        let robot_position = robot.position_within_space(&self.size);
                        match i {
                            0 => robot_position.x < x_middle && robot_position.y < y_middle,
                            1 => robot_position.x > x_middle && robot_position.y < y_middle,
                            2 => robot_position.x < x_middle && robot_position.y > y_middle,
                            3 => robot_position.x > x_middle && robot_position.y > y_middle,
                            _ => panic!(),
                        }
                    })
                    .count() as u64;
                n
            })
            .product()
    }
    fn display(&self) -> String {
        let mut string = String::with_capacity((self.size.x * self.size.y) as usize);

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                string.push(
                    match self
                        .robots
                        .iter()
                        .find(|robot| robot.position_within_space(&self.size) == Vector::new(x, y))
                        .is_some()
                    {
                        false => match x % 2 == 0 {
                            true => '.',
                            false => '_',
                        },
                        true => 'X',
                    },
                );
            }
            string.push('\n');
        }

        string
    }
}

#[derive(Debug, PartialEq)]
struct Robot {
    position: Vector,
    velocity: Vector,
}
impl Robot {
    fn step(&mut self) {
        self.position += self.velocity.clone();
    }
    fn position_within_space(&self, space: &Vector) -> Vector {
        let new_pos = Vector::new(
            ((self.position.x % space.x) + space.x) % space.x,
            ((self.position.y % space.y) + space.y) % space.y,
        );
        new_pos
    }
}

fn parse_robots(robots: &str) -> Vec<Robot> {
    robots
        .trim()
        .lines()
        .map(|line| {
            let mut vectors = line
                .trim()
                .split('v')
                .enumerate()
                .map(|(i, s)| match i % 2 == 0 {
                    true => Vector::from_str(&s[2..]),
                    false => Vector::from_str(&s[1..]),
                });
            Robot {
                position: vectors.next().unwrap(),
                velocity: vectors.next().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
struct Vector {
    x: i64,
    y: i64,
}
impl Vector {
    fn new(x: i64, y: i64) -> Vector {
        Vector { x, y }
    }
    fn from_str(s: &str) -> Vector {
        let mut numbers = s.split(',').map(|n| n.trim().parse().unwrap());

        Vector {
            x: numbers.next().unwrap(),
            y: numbers.next().unwrap(),
        }
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
    fn parse_robots_test() {
        let robots = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1";

        assert_eq!(
            parse_robots(robots),
            vec![
                Robot {
                    position: Vector::new(0, 4),
                    velocity: Vector::new(3, -3),
                },
                Robot {
                    position: Vector::new(6, 3),
                    velocity: Vector::new(-1, -3),
                },
                Robot {
                    position: Vector::new(10, 3),
                    velocity: Vector::new(-1, 2),
                },
                Robot {
                    position: Vector::new(2, 0),
                    velocity: Vector::new(2, -1),
                },
            ]
        );
    }

    #[test]
    fn example_test() {
        let robots = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        assert_eq!(solve(robots, Vector::new(11, 7)), 12);
    }
}
