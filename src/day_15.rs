pub fn run(mut dir: String) {
    dir.push_str("/day_15.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data);
    println!("{result}");
}

fn solve(data: &str) -> usize {
    let data = data.trim();

    let map: String = data
        .split_inclusive('\n')
        .take_while(|line| !line.trim().is_empty())
        .collect();
    let mut map = parse_map(&map);

    let instructions: String = data
        .split_inclusive('\n')
        .skip_while(|line| !line.trim().is_empty())
        .collect();
    let mut instructions = parse_instructions(&instructions);

    while let Some(instruction) = instructions.next() {
        map.move_robot(instruction);
    }

    map.gps_sum()
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Space {
    Wall,
    Box,
    Empty,
}
#[derive(Clone, Copy)]
enum Instruction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
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
                line.iter().enumerate().map(move |(x, c)| match c {
                    Space::Box => y * 100 + x,
                    _ => 0,
                })
            })
            .flatten()
            .sum()
    }
    fn move_robot(&mut self, dir: Instruction) {
        let new_pos = match dir {
            Instruction::Up => (self.robot_pos.0, self.robot_pos.1 - 1),
            Instruction::Down => (self.robot_pos.0, self.robot_pos.1 + 1),
            Instruction::Left => (self.robot_pos.0 - 1, self.robot_pos.1),
            Instruction::Right => (self.robot_pos.0 + 1, self.robot_pos.1),
        };
        if self.push(new_pos, dir) {
            self.robot_pos = new_pos;
        }
    }
    fn push(&mut self, pos: (usize, usize), dir: Instruction) -> bool {
        match self.map.get(pos.1).unwrap().get(pos.0).unwrap() {
            Space::Empty => true,
            Space::Wall => false,
            Space::Box => {
                if self.push(
                    match dir {
                        Instruction::Up => (pos.0, pos.1 - 1),
                        Instruction::Down => (pos.0, pos.1 + 1),
                        Instruction::Left => (pos.0 - 1, pos.1),
                        Instruction::Right => (pos.0 + 1, pos.1),
                    },
                    dir,
                ) {
                    *self.map.get_mut(pos.1).unwrap().get_mut(pos.0).unwrap() = Space::Empty;

                    match dir {
                        Instruction::Up => {
                            *self.map.get_mut(pos.1 - 1).unwrap().get_mut(pos.0).unwrap() =
                                Space::Box
                        }
                        Instruction::Down => {
                            *self.map.get_mut(pos.1 + 1).unwrap().get_mut(pos.0).unwrap() =
                                Space::Box
                        }
                        Instruction::Left => {
                            *self.map.get_mut(pos.1).unwrap().get_mut(pos.0 - 1).unwrap() =
                                Space::Box
                        }
                        Instruction::Right => {
                            *self.map.get_mut(pos.1).unwrap().get_mut(pos.0 + 1).unwrap() =
                                Space::Box
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }
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
                        'O' => Space::Box,
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
    fn push_test() {
        let mut map = Map {
            map: vec![
                vec![Space::Wall],
                vec![Space::Empty],
                vec![Space::Box],
                vec![Space::Box],
                vec![Space::Empty],
            ],
            robot_pos: (0, 4),
        };

        map.move_robot(Instruction::Up);
        assert_eq!(
            map.map,
            vec![
                vec![Space::Wall],
                vec![Space::Box],
                vec![Space::Box],
                vec![Space::Empty],
                vec![Space::Empty],
            ]
        );

        map.move_robot(Instruction::Up);
        assert_eq!(
            map.map,
            vec![
                vec![Space::Wall],
                vec![Space::Box],
                vec![Space::Box],
                vec![Space::Empty],
                vec![Space::Empty],
            ]
        );
    }

    #[test]
    fn gps_sum_test() {
        let map = parse_map(
            "#######
#...O..
#......",
        );
        println!("{:?}", map);

        assert_eq!(map.gps_sum(), 104);
    }

    #[test]
    fn small_example_test() {
        let example = "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        assert_eq!(solve(example), 2028);
    }

    #[test]
    fn example_test() {
        let example = "
##########
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

        assert_eq!(solve(example), 10092);
    }
}
