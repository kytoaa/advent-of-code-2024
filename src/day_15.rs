pub fn run(mut dir: String) {
    dir.push_str("/day_15.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data);
    println!("{result}");
}

fn solve(data: &str) -> usize {
    let data = data.trim();

    let map: String = {
        let mut map = String::new();
        let mut lines = data
            .lines()
            .take_while(|line| !line.trim().is_empty())
            .map(to_wide_map);
        while let Some(s) = lines.next() {
            map.push_str(&s);
            map.push('\n');
        }
        map
    };
    let mut map = parse_map(&map);
    let box_count = map.box_count();

    let instructions: String = data
        .split_inclusive('\n')
        .skip_while(|line| !line.trim().is_empty())
        .collect();
    let mut instructions = parse_instructions(&instructions);

    let step = 1;
    'outer: loop {
        for _ in 0..step {
            if let Some(instruction) = instructions.next() {
                map.move_robot(instruction);
            } else {
                break 'outer;
            }
        }
    }

    assert_eq!(map.box_count(), box_count);
    map.gps_sum()
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Space {
    Wall,
    Box(char),
    Empty,
}
#[derive(PartialEq, Clone, Copy, Debug)]
enum Instruction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Debug)]
struct Map {
    map: Vec<Vec<Space>>,
    robot_pos: (usize, usize),
}
impl Map {
    fn gps_sum(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == Space::Box('['))
                    .map(move |(x, _)| y * 100 + x)
            })
            .flatten()
            .sum()
    }
    fn box_count(&self) -> usize {
        self.map
            .iter()
            .map(|line| line.iter().filter(|c| **c == Space::Box('[')).count())
            .sum()
    }
    fn move_robot(&mut self, dir: Instruction) {
        let new_pos = match dir {
            Instruction::Up => (self.robot_pos.0, self.robot_pos.1 - 1),
            Instruction::Down => (self.robot_pos.0, self.robot_pos.1 + 1),
            Instruction::Left => (self.robot_pos.0 - 1, self.robot_pos.1),
            Instruction::Right => (self.robot_pos.0 + 1, self.robot_pos.1),
        };
        let check = self.push_and_check(new_pos, dir, &mut vec![]);
        if let Some(f) = check {
            f(self, dir);
            self.robot_pos = new_pos;
        }
    }
    fn push_and_check(
        &mut self,
        pos: (usize, usize),
        dir: Instruction,
        checked: &mut Vec<(usize, usize)>,
    ) -> Option<Box<dyn FnOnce(&mut Self, Instruction)>> {
        if checked.contains(&pos) {
            return Some(Box::new(|_, _| ()));
        }
        match self.map.get(pos.1).unwrap().get(pos.0).unwrap() {
            Space::Empty => Some(Box::new(|_, _| ())),
            Space::Wall => None,
            Space::Box(side) => {
                let side = *side;
                checked.push(pos.clone());
                let front_check = self.push_and_check(
                    match dir {
                        Instruction::Up => (pos.0, pos.1 - 1),
                        Instruction::Down => (pos.0, pos.1 + 1),
                        Instruction::Left => (pos.0 - 1, pos.1),
                        Instruction::Right => (pos.0 + 1, pos.1),
                    },
                    dir,
                    checked,
                );
                if front_check.is_none() {
                    return None;
                }
                let side_check = match dir {
                    Instruction::Up | Instruction::Down => {
                        let side_check = self.push_and_check(
                            {
                                let offset = match side {
                                    '[' => 1,
                                    ']' => -1,
                                    c => panic!("{c} is not a box side"),
                                };
                                ((pos.0 as isize + offset) as usize, pos.1)
                            },
                            dir,
                            checked,
                        );
                        if side_check.is_none() {
                            return None;
                        }
                        side_check
                    }
                    _ => None,
                };
                Some(Box::new(move |s, dir| {
                    if let Some(f) = front_check {
                        f(s, dir);
                    }
                    if let Some(f) = side_check {
                        f(s, dir);
                    }
                    let moved = *s.map.get(pos.1).unwrap().get(pos.0).unwrap();
                    *s.map.get_mut(pos.1).unwrap().get_mut(pos.0).unwrap() = Space::Empty;

                    match dir {
                        Instruction::Up => {
                            *s.map.get_mut(pos.1 - 1).unwrap().get_mut(pos.0).unwrap() = moved;
                        }
                        Instruction::Down => {
                            *s.map.get_mut(pos.1 + 1).unwrap().get_mut(pos.0).unwrap() = moved;
                        }
                        Instruction::Left => {
                            *s.map.get_mut(pos.1).unwrap().get_mut(pos.0 - 1).unwrap() = moved;
                        }
                        Instruction::Right => {
                            *s.map.get_mut(pos.1).unwrap().get_mut(pos.0 + 1).unwrap() = moved
                        }
                    }
                }))
            }
        }
    }
    #[allow(dead_code)]
    fn display(&self) -> String {
        let mut string = String::new();

        for (y, line) in self.map.iter().enumerate() {
            for (x, space) in line.iter().enumerate() {
                if self.robot_pos == (x, y) {
                    string.push('@');
                    continue;
                }
                string.push(match space {
                    Space::Wall => '#',
                    Space::Box(c) => *c,
                    Space::Empty => '.',
                });
            }
            string.push('\n');
        }

        string
    }
}

fn to_wide_map(map: &str) -> String {
    map.replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.")
}

fn parse_map(map: &str) -> Map {
    let mut robot_pos = (0, 0);
    let map = map
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '@' {
                        robot_pos = (x, y);
                    }
                    match c {
                        '#' => Space::Wall,
                        '[' => Space::Box('['),
                        ']' => Space::Box(']'),
                        '.' | '@' => Space::Empty,
                        c => panic!("{c} is not a tile"),
                    }
                })
                .collect()
        })
        .collect();

    Map { map, robot_pos }
}

fn parse_instructions<'a>(instructions: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    instructions
        .trim()
        .chars()
        .filter(|c| match c {
            '^' | 'v' | '<' | '>' => true,
            _ => false,
        })
        .map(|c| match c {
            '^' => Instruction::Up,
            'v' => Instruction::Down,
            '<' => Instruction::Left,
            '>' => Instruction::Right,
            c => panic!("{c} is not a direction"),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gps_sum_test() {
        let map = parse_map(
            "
##########
##...[]...
##........",
        );

        assert_eq!(map.gps_sum(), 105);
    }

    #[test]
    fn push_test() {
        let mut map = parse_map(
            "..#
...
.[]
[].
@..",
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "..#
.[]
[].
@..
...",
            )
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "..#
.[]
[].
@..
...",
            )
        );
    }
    #[test]
    fn complex_push_tests() {
        let mut map = parse_map(
            "
...#
[]..
..[]
.[].
.@.."
                .trim(),
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "
...#
[][]
.[].
.@..
...."
                    .trim(),
            )
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "
...#
[][]
.[].
.@..
...."
                    .trim(),
            )
        );

        let mut map = parse_map(
            "
....
.[]#
[]..
..[]
.[].
.@.."
                .trim(),
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "
....
.[]#
[][]
.[].
.@..
...."
                    .trim(),
            )
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "
....
.[]#
[][]
.[].
.@..
...."
                    .trim(),
            )
        );

        let mut map = parse_map(
            "
....
...#
.[].
[][]
.[].
.@.."
                .trim(),
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "
....
.[]#
[][]
.[].
.@..
...."
                    .trim(),
            )
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "
....
.[]#
[][]
.[].
.@..
...."
                    .trim(),
            )
        );

        let mut map = parse_map(
            "
....
...#
.[].
..[]
.[].
.[].
.@.."
                .trim(),
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "
....
.[]#
..[]
.[].
.[].
.@..
...."
                    .trim(),
            )
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "
....
.[]#
..[]
.[].
.[].
.@..
...."
                    .trim(),
            )
        );

        let mut map = parse_map(
            "
......
..#..#
......
[][][]
.[][].
..[]..
..@..."
                .trim(),
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "
......
..#..#
[][][]
.[][].
..[]..
..@...
......"
            )
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map,
            parse_map(
                "
......
..#..#
[][][]
.[][].
..[]..
..@...
......"
            )
        );
    }

    #[test]
    fn example_test() {
        let example = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        assert_eq!(solve(example), 9021);
    }
}
