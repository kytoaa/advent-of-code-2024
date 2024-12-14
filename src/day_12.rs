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
        .map(|region| region.len() * sides(&region))
        .sum()
}

type Coord = (usize, usize);
type Region = Vec<Coord>;

fn sides(region: &Region) -> usize {
    if region.len() == 1 || region.len() == 2 {
        return 4;
    }
    let outer_corners: usize = region
        .iter()
        .map(|pos| exterior_corner_count(region, *pos))
        .sum();

    let inner_corners: usize = region
        .iter()
        .map(|pos| interior_corner_count(region, *pos))
        .sum();

    outer_corners + inner_corners
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

fn touching(a: Coord, b: Coord) -> Option<usize> {
    if a.0 == b.0 && a.1.abs_diff(b.1) == 1 {
        Some(0)
    } else if a.0.abs_diff(b.0) == 1 && a.1 == b.1 {
        Some(1)
    } else {
        None
    }
}
#[allow(dead_code)]
fn touching_count(r: &Region, c: Coord) -> usize {
    r.iter()
        .filter(|coord| touching(c, **coord).is_some())
        .count()
}

fn exterior_corner_count(r: &Region, c: Coord) -> usize {
    let directions = r.iter().filter_map(|coord| touching(c, *coord)).fold(
        [0, 0],
        |mut side_directions, side| {
            side_directions[side] += 1;
            side_directions
        },
    );
    if directions.iter().sum::<usize>() == 1 {
        return 2;
    }
    if !directions.contains(&2) {
        return 1;
    }
    return 0;
}
fn interior_corner_count(r: &Region, c: Coord) -> usize {
    let mut corners = vec![];

    if c.1 > 0 {
        corners.push(Some((c.0, c.1 - 1)));
        corners.push(Some((c.0 + 1, c.1 - 1)));
    } else {
        corners.push(None);
        corners.push(None);
    }
    corners.push(Some((c.0 + 1, c.1)));
    corners.push(Some((c.0 + 1, c.1 + 1)));
    corners.push(Some((c.0, c.1 + 1)));
    if c.0 > 0 {
        corners.push(Some((c.0 - 1, c.1 + 1)));
        corners.push(Some((c.0 - 1, c.1)));
        if c.1 > 0 {
            corners.push(Some((c.0 - 1, c.1 - 1)));
        } else {
            corners.push(None);
        }
    } else {
        corners.push(None);
        corners.push(None);
        corners.push(None);
    }

    let mut corner_count = 0;

    // hate this but im sick of this problem and its just the easiest way
    {
        let corner = corners[1];
        if corner.is_none() || !r.contains(&corner.unwrap()) {
            if corners[0].is_some()
                && r.contains(&corners[0].unwrap())
                && corners[2].is_some()
                && r.contains(&corners[2].unwrap())
            {
                corner_count += 1;
            }
        }
    }
    {
        let corner = corners[3];
        if corner.is_none() || !r.contains(&corner.unwrap()) {
            if corners[2].is_some()
                && r.contains(&corners[2].unwrap())
                && corners[4].is_some()
                && r.contains(&corners[4].unwrap())
            {
                corner_count += 1;
            }
        }
    }
    {
        let corner = corners[5];
        if corner.is_none() || !r.contains(&corner.unwrap()) {
            if corners[4].is_some()
                && r.contains(&corners[4].unwrap())
                && corners[6].is_some()
                && r.contains(&corners[6].unwrap())
            {
                corner_count += 1;
            }
        }
    }
    {
        let corner = corners[7];
        if corner.is_none() || !r.contains(&corner.unwrap()) {
            if corners[0].is_some()
                && r.contains(&corners[0].unwrap())
                && corners[6].is_some()
                && r.contains(&corners[6].unwrap())
            {
                corner_count += 1;
            }
        }
    }

    corner_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let map = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        assert_eq!(solve(map), 1206);
    }

    #[test]
    fn smaller_examples_test() {
        let map = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(solve(map), 80);

        let map = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(solve(map), 236);

        println!("last area");

        let map = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(solve(map), 368);
    }

    #[test]
    fn sides_test() {
        assert_eq!(sides(&vec![(0, 0), (1, 0), (2, 0), (3, 0)]), 4);
        assert_eq!(sides(&vec![(2, 1), (2, 2), (3, 2), (3, 3)]), 8);

        assert_eq!(
            sides(&vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]),
            4
        );
    }
}
