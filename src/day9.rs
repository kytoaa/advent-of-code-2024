pub fn run(mut dir: String) {
    dir.push_str("/day9.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data);
    println!("checksum: {result}");
}

fn solve(data: &str) -> u64 {
    let mut file_system = parse_file_system(data);

    defragment_file_system(&mut file_system);

    checksum(&file_system)
}

type FileID = u64;
type FileSystem = Vec<Option<FileID>>;

fn checksum(file_system: &FileSystem) -> u64 {
    file_system
        .iter()
        .enumerate()
        .filter_map(|(i, o)| o.map(|id| id * i as u64))
        .sum()
}

fn defragment_file_system(file_system: &mut FileSystem) {
    let empty_count = file_system.iter().filter(|f| f == &&None).count();

    let mut end_full = file_system.len() - 1;
    let mut start_empty = 0;

    for _ in 0..empty_count {
        loop {
            if file_system[start_empty].is_none() {
                break;
            }
            start_empty += 1;
        }
        loop {
            if file_system[end_full].is_some() {
                break;
            }
            end_full -= 1;
        }
        if end_full <= start_empty {
            break;
        }
        file_system.swap(start_empty, end_full);
    }
}

fn parse_file_system(data: &str) -> FileSystem {
    let mut file_system = vec![];
    let mut file_id = 0;
    data.trim().chars().enumerate().for_each(|(i, c)| {
        if i % 2 == 0 {
            for _ in 0..c.to_digit(10).unwrap() {
                file_system.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..c.to_digit(10).unwrap() {
                file_system.push(None);
            }
        }
    });
    file_system
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_example_test() {
        let example = "12345";

        assert_eq!(
            parse_file_system(example),
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2)
            ]
        );
    }
    #[test]
    fn defragment_file_example_test() {
        let mut example = parse_file_system("12345");

        defragment_file_system(&mut example);

        assert_eq!(
            example,
            vec![
                Some(0),
                Some(2),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
            ]
        );
    }
    #[test]
    fn defragment_file_long_example_test() {
        let mut example = parse_file_system("2333133121414131402");

        defragment_file_system(&mut example);

        assert_eq!(
            example,
            vec![
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(8),
                Some(1),
                Some(1),
                Some(1),
                Some(8),
                Some(8),
                Some(8),
                Some(2),
                Some(7),
                Some(7),
                Some(7),
                Some(3),
                Some(3),
                Some(3),
                Some(6),
                Some(4),
                Some(4),
                Some(6),
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(6),
                Some(6),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ]
        );
    }

    #[test]
    fn example_checksum_test() {
        let mut example = parse_file_system("2333133121414131402");

        defragment_file_system(&mut example);

        assert_eq!(checksum(&example), 1928);
    }
}
