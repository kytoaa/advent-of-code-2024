pub fn run(mut dir: String) {
    dir.push_str("/day_12.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data);
    println!("{result}");
}

fn solve(data: &str) -> usize {
    let regions = parse_map(data);

    regions
        .into_iter()
        .map(|region| region.len() * perimeter(&region))
        .sum()
}

type Coord = (usize, usize);
type Region = Vec<Coord>;

fn perimeter(region: &Region) -> usize {
    region
        .iter()
        .map(|c| 4 - region.iter().filter(|other| touching(*c, **other)).count())
        .sum()
}

fn parse_map(map: &str) -> Vec<Region> {
    let mut map = map
        .trim()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(_, line)| line.chars().map(|c| Some(c)).collect())
        .collect::<Vec<Vec<Option<char>>>>();

    unsafe { &*(&map as *const Vec<Vec<Option<char>>>) }
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    Some(c) => Some(find_region((x, y), *c, unsafe {
                        &mut *(&mut map as *mut Vec<Vec<Option<char>>>)
                    })),
                    None => None,
                })
                .collect::<Vec<Region>>()
        })
        .flatten()
        .collect()
}

fn find_region(pos: Coord, c: char, map: &mut Vec<Vec<Option<char>>>) -> Region {
    if pos.0 >= map[0].len() || pos.1 >= map[0].len() {
        return vec![];
    }
    let mut region = vec![pos];
    if c != map[pos.1][pos.0].unwrap_or('\0') {
        return vec![];
    }
    _ = map[pos.1][pos.0].take();

    region.append(&mut find_region((pos.0 + 1, pos.1), c, map));
    if pos.0 > 0 {
        region.append(&mut find_region((pos.0 - 1, pos.1), c, map));
    }
    region.append(&mut find_region((pos.0, pos.1 + 1), c, map));
    if pos.1 > 0 {
        region.append(&mut find_region((pos.0, pos.1 - 1), c, map));
    }

    return region;
}

fn touching(a: Coord, b: Coord) -> bool {
    (a.0 == b.0 && a.1.abs_diff(b.1) == 1) || (a.0.abs_diff(b.0) == 1 && a.1 == b.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let map = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        assert_eq!(solve(map), 1930);
    }

    #[test]
    fn perimeter_test() {
        assert_eq!(perimeter(&vec![(0, 0), (1, 0), (2, 0), (3, 0)]), 10);
        assert_eq!(perimeter(&vec![(2, 1), (2, 2), (3, 2), (3, 3)]), 10);
    }
}
