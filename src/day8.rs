use std::collections::HashSet;

#[allow(dead_code)]
pub fn run(mut dir: String) {
    dir.push_str("/day8.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data);
    println!("{}", result);
}

fn solve(data: &str) -> u32 {
    let map = parse_map(data);

    map.get_antinode_count()
}

type Coord = (isize, isize);
type Antenna = (char, Coord);

#[derive(Debug, PartialEq)]
struct Map {
    size: (usize, usize),
    antennas: Vec<Antenna>,
}
impl Map {
    fn get_antinode_count(&self) -> u32 {
        self.antennas
            .iter()
            .map(|a| self.antinode_count_for_antenna(*a))
            .flatten()
            .collect::<HashSet<Coord>>()
            .len() as u32
    }
    fn antinode_count_for_antenna(&self, antenna: Antenna) -> Vec<Coord> {
        let mut antennas = vec![];

        for other_antenna in &self.antennas {
            if *other_antenna == antenna {
                continue;
            }
            if other_antenna.0 != antenna.0 {
                continue;
            }

            let dif = (
                other_antenna.1 .0 - antenna.1 .0,
                other_antenna.1 .1 - antenna.1 .1,
            );

            for i in 0..isize::MAX {
                let other = (antenna.1 .0 - (dif.0 * i), antenna.1 .1 - (dif.1 * i));

                if other.0 < 0 || other.0 >= self.size.0 as isize {
                    break;
                }
                if other.1 < 0 || other.1 >= self.size.1 as isize {
                    break;
                }

                antennas.push(other);
            }
        }

        antennas
    }
}

fn parse_map(data: &str) -> Map {
    let lines: Vec<&str> = data.trim().lines().map(|line| line.trim()).collect();

    Map {
        size: (lines.first().unwrap().len(), lines.len()),
        antennas: lines
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, v)| {
                    if v.is_alphanumeric() {
                        Some((v, (x as isize, y as isize)))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let data = "..a.
a...
...0
.0..";

        assert_eq!(
            parse_map(data),
            Map {
                size: (4, 4),
                antennas: vec![('a', (2, 0)), ('a', (0, 1)), ('0', (3, 2)), ('0', (1, 3))]
            }
        )
    }

    #[test]
    fn example_test() {
        let data = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(solve(data), 34);
    }
}
