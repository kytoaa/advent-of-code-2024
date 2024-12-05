pub fn run(mut dir: String) {
    dir.push_str("/day4.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data[..]);
    println!("{}", result);

    let result = solve_x_mas(&data[..]);
    println!("{}", result);
}

fn solve(data: &str) -> u32 {
    let wordsearch = to_wordsearch(data);

    search_horizontal(&wordsearch)
        + search_horizontal_reverse(&wordsearch)
        + search_vertical(&wordsearch)
        + search_vertical_reverse(&wordsearch)
        + search_diagonal_down(&wordsearch)
        + search_diagonal_down_reverse(&wordsearch)
        + search_diagonal_up(&wordsearch)
        + search_diagonal_up_reverse(&wordsearch)
}
fn solve_x_mas(data: &str) -> u32 {
    let wordsearch = to_wordsearch(data);

    search_x_mas(&wordsearch)
}

type Wordsearch = Vec<Vec<char>>;

fn to_wordsearch(wordsearch: &str) -> Wordsearch {
    wordsearch
        .trim()
        .split('\n')
        .map(|s| s.trim().to_lowercase().chars().collect())
        .collect()
}

fn search_x_mas(wordsearch: &Wordsearch) -> u32 {
    wordsearch.into_iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row.into_iter().enumerate().fold(0, |acc, (j, c)| {
            match wordsearch.get(i + 1).map(|row| row.get(j + 1)) {
                Some(Some('a')) => (),
                _ => return acc,
            }
            match *c {
                'm' => match wordsearch.get(i + 2).map(|row| row.get(j + 2)) {
                    Some(Some('s')) => (),
                    _ => return acc,
                },
                's' => match wordsearch.get(i + 2).map(|row| row.get(j + 2)) {
                    Some(Some('m')) => (),
                    _ => return acc,
                },
                _ => return acc,
            }
            match row.get(j + 2) {
                Some('m') => match wordsearch.get(i + 2).map(|row| row.get(j)) {
                    Some(Some('s')) => (),
                    _ => return acc,
                },
                Some('s') => match wordsearch.get(i + 2).map(|row| row.get(j)) {
                    Some(Some('m')) => (),
                    _ => return acc,
                },
                _ => return acc,
            }
            acc + 1
        })
    })
}

fn search_horizontal(wordsearch: &Wordsearch) -> u32 {
    wordsearch
        .iter()
        .map(|row| xmas_count(row.iter().map(|c| *c)))
        .sum()
}
fn search_horizontal_reverse(wordsearch: &Wordsearch) -> u32 {
    wordsearch
        .iter()
        .map(|row| xmas_count(row.iter().map(|c| *c).rev()))
        .sum()
}

fn search_vertical(wordsearch: &Wordsearch) -> u32 {
    let columns = wordsearch.len();
    let mut total = 0;

    for column in 0..columns {
        total += xmas_count(wordsearch.iter().map(|row| row[column]));
    }

    total
}
fn search_vertical_reverse(wordsearch: &Wordsearch) -> u32 {
    let columns = wordsearch.len();
    let mut total = 0;

    for column in 0..columns {
        total += xmas_count(wordsearch.iter().map(|row| row[column]).rev());
    }

    total
}

fn search_diagonal_down(wordsearch: &Wordsearch) -> u32 {
    ((-(wordsearch.len() as isize))..(wordsearch.len() as isize))
        .map(|i| {
            wordsearch.iter().enumerate().filter_map(move |(j, row)| {
                let c = row.get(match i + (j as isize) {
                    i if i < 0 => return None,
                    i => i as usize,
                });
                c.map(|c| *c)
            })
        })
        .map(|diag| xmas_count(diag))
        .sum()
}
fn search_diagonal_down_reverse(wordsearch: &Wordsearch) -> u32 {
    ((-(wordsearch.len() as isize))..(wordsearch.len() as isize))
        .map(|i| {
            wordsearch.iter().enumerate().filter_map(move |(j, row)| {
                let c = row.get(match i + (j as isize) {
                    i if i < 0 => return None,
                    i => i as usize,
                });
                c.map(|c| *c)
            })
        })
        .map(|diag| xmas_count(diag.rev()))
        .sum()
}

fn search_diagonal_up(wordsearch: &Wordsearch) -> u32 {
    ((-(wordsearch.len() as isize))..(wordsearch.len() as isize))
        .map(|i| {
            wordsearch.iter().enumerate().filter_map(move |(j, row)| {
                let j = row.len() - 1 - j;
                let c = row.get(match i + (j as isize) {
                    i if i < 0 => return None,
                    i => i as usize,
                });
                c.map(|c| *c)
            })
        })
        .map(|diag| xmas_count(diag.rev()))
        .sum()
}
fn search_diagonal_up_reverse(wordsearch: &Wordsearch) -> u32 {
    ((-(wordsearch.len() as isize))..(wordsearch.len() as isize))
        .map(|i| {
            wordsearch.iter().enumerate().filter_map(move |(j, row)| {
                let j = row.len() - 1 - j;
                let c = row.get(match i + (j as isize) {
                    i if i < 0 => return None,
                    i => i as usize,
                });
                c.map(|c| *c)
            })
        })
        .map(|diag| xmas_count(diag))
        .sum()
}

fn xmas_count(chars: impl Iterator<Item = char>) -> u32 {
    let mut chars = chars.peekable();
    let mut total_xmas = 0;

    loop {
        match chars.next() {
            Some('x') => (),
            Some(_) => continue,
            None => return total_xmas,
        }
        match chars.peek() {
            Some('m') => _ = chars.next(),
            Some(_) => continue,
            None => return total_xmas,
        }
        match chars.peek() {
            Some('a') => _ = chars.next(),
            Some(_) => continue,
            None => return total_xmas,
        }
        match chars.peek() {
            Some('s') => _ = chars.next(),
            Some(_) => continue,
            None => return total_xmas,
        }
        total_xmas += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xmas_count_test() {
        let xmas = "xmas";
        assert_eq!(xmas_count(xmas.chars()), 1);

        let xmas = "xmasxmasxma";
        assert_eq!(xmas_count(xmas.chars()), 2);

        let xmas = "x_masxmasxma";
        assert_eq!(xmas_count(xmas.chars()), 1);

        let xmas = "xmaxmas";
        assert_eq!(xmas_count(xmas.chars()), 1);
    }

    #[test]
    fn horizontal_xmas_tests() {
        let wordsearch = to_wordsearch(
            "
xmas
xma.
samx
xmas",
        );
        assert_eq!(search_horizontal(&wordsearch), 2);
        assert_eq!(search_horizontal_reverse(&wordsearch), 1);
    }
    #[test]
    fn vertical_xmas_tests() {
        let wordsearch = to_wordsearch(
            "
xxsx
mmam
aama
smxs",
        );
        assert_eq!(search_vertical(&wordsearch), 2);
        assert_eq!(search_vertical_reverse(&wordsearch), 1);
    }
    #[test]
    fn diagonal_xmas_tests() {
        let wordsearch = to_wordsearch(
            "
xxsx
mmam
aaaa
smxs",
        );

        assert_eq!(search_diagonal_down(&wordsearch), 1);

        let wordsearch = to_wordsearch(
            "
sxsx
maam
aama
smxx",
        );

        assert_eq!(search_diagonal_down_reverse(&wordsearch), 1);

        let wordsearch = to_wordsearch(
            "
sxss
maam
amma
xmxx",
        );

        assert_eq!(search_diagonal_up(&wordsearch), 1);

        let wordsearch = to_wordsearch(
            "
sxsx
mamm
aama
smxx",
        );

        assert_eq!(search_diagonal_up_reverse(&wordsearch), 1);
    }

    #[test]
    fn example_test() {
        let wordsearch = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(solve(wordsearch), 18);
    }

    #[test]
    fn example_x_mas_test() {
        let wordsearch = to_wordsearch(
            "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );

        assert_eq!(search_x_mas(&wordsearch), 9);
    }
}
