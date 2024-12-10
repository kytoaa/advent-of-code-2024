pub fn run(mut dir: String) {
    dir.push_str("/day_10.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data);
    println!("{result}");
}

fn solve(data: &str) -> u32 {
    heightmap_to_paths(&parse_heightmap(data))
        .into_iter()
        .map(|path| path.score(std::rc::Rc::new(std::cell::RefCell::new(vec![]))))
        .sum()
}

fn heightmap_to_paths(heightmap: &Vec<Vec<Height>>) -> Vec<Box<PathNode>> {
    heightmap
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, n)| {
                if *n == 0 {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .flatten()
        .filter_map(|pos| PathNode::extend(heightmap, pos, None))
        .collect()
}

type Height = i32;

#[derive(Debug)]
struct PathNode {
    height: Height,
    position: (isize, isize),
    left: Option<Box<PathNode>>,
    right: Option<Box<PathNode>>,
    up: Option<Box<PathNode>>,
    down: Option<Box<PathNode>>,
}
impl PathNode {
    fn extend(
        map: &Vec<Vec<Height>>,
        pos: (isize, isize),
        prev: Option<Height>,
    ) -> Option<Box<PathNode>> {
        if pos.0 < 0 || pos.0 >= map.len() as isize {
            return None;
        }
        if pos.1 < 0 || pos.1 >= map[0].len() as isize {
            return None;
        }
        let height = map[pos.1 as usize][pos.0 as usize];
        match prev {
            Some(n) => {
                if height - n != 1 {
                    return None;
                }
            }
            None => {
                if height != 0 {
                    panic!();
                }
            }
        }

        Some(Box::new(PathNode {
            height,
            position: pos,
            left: PathNode::extend(map, (pos.0 - 1, pos.1), Some(height)),
            right: PathNode::extend(map, (pos.0 + 1, pos.1), Some(height)),
            up: PathNode::extend(map, (pos.0, pos.1 - 1), Some(height)),
            down: PathNode::extend(map, (pos.0, pos.1 + 1), Some(height)),
        }))
    }
    // not a fan of the rc refcell but its the easiest way to do this without more major changes
    fn score(&self, found: std::rc::Rc<std::cell::RefCell<Vec<(isize, isize)>>>) -> u32 {
        if self.height == 9 {
            if found.borrow_mut().contains(&self.position) {
                0
            } else {
                found.borrow_mut().push(self.position);
                1
            }
        } else {
            self.left
                .as_ref()
                .map(|p| p.score(found.clone()))
                .unwrap_or(0)
                + self
                    .right
                    .as_ref()
                    .map(|p| p.score(found.clone()))
                    .unwrap_or(0)
                + self
                    .up
                    .as_ref()
                    .map(|p| p.score(found.clone()))
                    .unwrap_or(0)
                + self
                    .down
                    .as_ref()
                    .map(|p| p.score(found.clone()))
                    .unwrap_or(0)
        }
    }
}

fn parse_heightmap(data: &str) -> Vec<Vec<Height>> {
    data.trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let example = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        assert_eq!(solve(example), 36);
    }

    #[test]
    fn short_example_test() {
        let example = "9990999
9991999
9992999
6543456
7999997
8199918
9999999";

        assert_eq!(solve(example), 2);
    }
}
