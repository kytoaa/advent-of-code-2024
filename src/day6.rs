pub fn run(mut dir: String) {
    dir.push_str("/day6.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data);
    println!("{result}");

    let loops = count_loops(&data);
    println!("{loops} possible loops");
}

fn solve(data: &str) -> u32 {
    let mut map = parse_map(data);

    loop {
        map = match map.step() {
            MapResult::Ok(m) => m,
            MapResult::LeftMap(m) | MapResult::Loop(m) => return m.count_traversed(),
        }
    }
}

fn count_loops(data: &str) -> u32 {
    let map = parse_map(data);
    let mut loops = 0;

    for x in 0..map.map.len() {
        for y in 0..map.map.len() {
            let mut map = map.clone();
            if let Some(space) = map.map.get_mut(x).map(|col| col.get_mut(y)).flatten() {
                match space {
                    Space::Empty(_) => *space = Space::Obstacle,
                    _ => (),
                }
                loops += loop {
                    match map.step() {
                        MapResult::Ok(m) => map = m,
                        MapResult::Loop(_) => break 1,
                        MapResult::LeftMap(_) => break 0,
                    }
                }
            }
        }
    }

    loops
}

fn parse_map(map: &str) -> Map {
    let columns: Vec<Vec<char>> = {
        let rows: Vec<Vec<char>> = map
            .trim()
            .split('\n')
            .map(|line| line.trim().chars().collect())
            .collect();

        rows.iter()
            .enumerate()
            .map(|(i, row)| row.iter().enumerate().map(|(j, _)| rows[j][i]).collect())
            .collect()
    };
    let guard_pos = {
        let mut pos = (0, 0);
        'find: for (x, col) in columns.iter().enumerate() {
            for (y, c) in col.iter().enumerate() {
                pos = (x, y);
                if *c == '^' {
                    break 'find;
                }
            }
        }
        pos
    };

    Map {
        guard: Guard((guard_pos.0 as isize, guard_pos.1 as isize), Direction::Up),
        map: columns
            .into_iter()
            .map(|col| {
                col.into_iter()
                    .map(|c| match c {
                        '#' => Space::Obstacle,
                        '^' => Space::Empty(Traversed::Yes(vec![Direction::Up])),
                        _ => Space::Empty(Traversed::No),
                    })
                    .collect()
            })
            .collect(),
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug, PartialEq, Clone)]
enum Traversed {
    Yes(Vec<Direction>),
    No,
}
#[derive(Debug, PartialEq, Clone)]
enum Space {
    Empty(Traversed),
    Obstacle,
}
#[derive(Debug, PartialEq, Clone)]
struct Guard((isize, isize), Direction);
impl Guard {
    fn turn(&self) -> Guard {
        Guard(
            self.0,
            match self.1 {
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
            },
        )
    }
    fn next_position(&self) -> (isize, isize) {
        match self.1 {
            Direction::Left => (self.0 .0 - 1, self.0 .1),
            Direction::Up => (self.0 .0, self.0 .1 - 1),
            Direction::Right => (self.0 .0 + 1, self.0 .1),
            Direction::Down => (self.0 .0, self.0 .1 + 1),
        }
    }
}

#[derive(Debug, PartialEq)]
enum MapResult {
    Ok(Map),
    LeftMap(Map),
    Loop(Map),
}
#[allow(dead_code)]
impl MapResult {
    fn unwrap(self) -> Map {
        match self {
            MapResult::Ok(m) => m,
            MapResult::LeftMap(m) => m,
            MapResult::Loop(m) => m,
        }
    }
    fn is_err(&self) -> bool {
        match self {
            MapResult::LeftMap(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Map {
    map: Vec<Vec<Space>>,
    guard: Guard,
}
impl Map {
    fn step(mut self) -> MapResult {
        let new_position = self.guard.next_position();
        let new_position = (
            if new_position.0 < 0 {
                return MapResult::LeftMap(self);
            } else {
                new_position.0 as usize
            },
            if new_position.1 < 0 {
                return MapResult::LeftMap(self);
            } else {
                new_position.1 as usize
            },
        );
        match self
            .map
            .get_mut(new_position.0)
            .map(|c| c.get_mut(new_position.1))
            .flatten()
        {
            None => return MapResult::LeftMap(self),
            Some(space) => match space {
                Space::Obstacle => self.guard = self.guard.turn(),
                Space::Empty(traversed) => {
                    self.guard = Guard(
                        (new_position.0 as isize, new_position.1 as isize),
                        self.guard.1,
                    );
                    match traversed {
                        Traversed::Yes(dirs) => {
                            if dirs.contains(&self.guard.1) {
                                return MapResult::Loop(self);
                            }
                            dirs.push(self.guard.1);
                        }
                        Traversed::No => *traversed = Traversed::Yes(vec![self.guard.1]),
                    }
                }
            },
        }
        MapResult::Ok(self)
    }
    fn count_traversed(&self) -> u32 {
        self.map
            .iter()
            .map(|col| {
                col.iter()
                    .map(|space| {
                        if let Space::Empty(Traversed::Yes(_)) = space {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_parse_test() {
        let map = "..#
#..
##^";
        assert_eq!(
            parse_map(map),
            Map {
                guard: Guard((2, 2), Direction::Up),
                map: vec![
                    vec![
                        Space::Empty(Traversed::No),
                        Space::Obstacle,
                        Space::Obstacle
                    ],
                    vec![
                        Space::Empty(Traversed::No),
                        Space::Empty(Traversed::No),
                        Space::Obstacle
                    ],
                    vec![
                        Space::Obstacle,
                        Space::Empty(Traversed::No),
                        Space::Empty(Traversed::Yes(vec![Direction::Up])),
                    ],
                ]
            }
        );
    }
    #[test]
    fn map_move_test() {
        let map = "..#
#..
##^";

        let map = parse_map(map);

        let map = map.step().unwrap();
        assert_eq!(
            map,
            Map {
                guard: Guard((2, 1), Direction::Up),
                map: vec![
                    vec![
                        Space::Empty(Traversed::No),
                        Space::Obstacle,
                        Space::Obstacle
                    ],
                    vec![
                        Space::Empty(Traversed::No),
                        Space::Empty(Traversed::No),
                        Space::Obstacle
                    ],
                    vec![
                        Space::Obstacle,
                        Space::Empty(Traversed::Yes(vec![Direction::Up])),
                        Space::Empty(Traversed::Yes(vec![Direction::Up])),
                    ],
                ]
            }
        );

        let map = map.step().unwrap();
        assert_eq!(
            map,
            Map {
                guard: Guard((2, 1), Direction::Right),
                map: vec![
                    vec![
                        Space::Empty(Traversed::No),
                        Space::Obstacle,
                        Space::Obstacle
                    ],
                    vec![
                        Space::Empty(Traversed::No),
                        Space::Empty(Traversed::No),
                        Space::Obstacle
                    ],
                    vec![
                        Space::Obstacle,
                        Space::Empty(Traversed::Yes(vec![Direction::Up])),
                        Space::Empty(Traversed::Yes(vec![Direction::Up])),
                    ],
                ]
            }
        );
        assert!(map.step().is_err());
    }

    #[test]
    fn example_test() {
        let map = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(solve(map), 41);
    }

    #[test]
    fn example_loop_test() {
        let map = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(count_loops(map), 6);
    }
}
