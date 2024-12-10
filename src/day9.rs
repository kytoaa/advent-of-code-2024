#[allow(dead_code)]
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

#[derive(Debug, PartialEq)]
enum File {
    File(FileID, usize),
    Empty(usize),
}
impl File {
    fn size(&self) -> usize {
        match self {
            File::File(_, s) => *s,
            File::Empty(s) => *s,
        }
    }
    fn is_file(&self) -> bool {
        match self {
            File::File(_, _) => true,
            File::Empty(_) => false,
        }
    }
}
type FileID = u64;
type FileSystem = Vec<File>;

fn checksum(file_system: &FileSystem) -> u64 {
    let mut i = 0;
    let mut total = 0;
    for file in file_system {
        total += match file {
            File::File(id, size) => {
                let mut n = 0;
                for j in 0..(*size) {
                    n += (i + j as u64) * *id;
                }
                n
            }
            File::Empty(_) => 0,
        };
        i += file.size() as u64;
    }
    total
}

fn defragment_file_system(file_system: &mut FileSystem) {
    let mut checked = vec![];
    loop {
        let (mut next_file_index, next_file_size) = {
            let next = match file_system
                .iter()
                .enumerate()
                .filter(|file| match file {
                    (_, File::File(_, _)) => true,
                    (_, File::Empty(_)) => false,
                })
                .rev()
                .filter(|(_, file)| match file {
                    File::File(id, _) => !checked.contains(id),
                    File::Empty(_) => panic!(),
                })
                .next()
            {
                Some(n) => n,
                None => break,
            };

            (
                next.0,
                *match next.1 {
                    File::File(id, i) => {
                        checked.push(*id);
                        i
                    }
                    File::Empty(_) => panic!(),
                },
            )
        };
        let (space_index, space_size) = match file_system.iter().enumerate().find(|(_, file)| {
            if let File::Empty(size) = file {
                *size >= next_file_size
            } else {
                false
            }
        }) {
            Some((i, file)) => (
                i,
                if let File::Empty(size) = file {
                    size
                } else {
                    panic!()
                },
            ),
            None => continue,
        };

        if space_index >= next_file_index {
            continue;
        }

        let size_difference = space_size - next_file_size;

        {
            let file = file_system.remove(next_file_index);
            file_system.insert(space_index, file);
        }
        let space_removed = file_system.remove(space_index + 1).size();

        if size_difference != 0 {
            if let Some(File::Empty(size)) = file_system.get_mut(space_index + 1) {
                *size += size_difference;
            } else {
                file_system.insert(space_index + 1, File::Empty(size_difference));
                next_file_index += 1;
            }
        }

        let empty_next_size = file_system
            .get(next_file_index)
            .filter(|f| !f.is_file())
            .map(|f| f.size());

        if let Some(File::Empty(size)) = file_system.get_mut(next_file_index - 1) {
            *size += space_removed - size_difference;

            if empty_next_size.is_some() {
                *size += empty_next_size.unwrap();
                _ = file_system.remove(next_file_index);
            }
        } else if let Some(File::Empty(size)) = file_system.get_mut(next_file_index) {
            *size += space_removed - size_difference;
        } else {
            file_system.insert(
                next_file_index,
                File::Empty(space_removed - size_difference),
            );
        }
    }
}

fn parse_file_system(data: &str) -> FileSystem {
    let mut file_system = vec![];
    let mut file_id = 0;

    data.trim().chars().enumerate().for_each(|(i, c)| {
        if i % 2 == 0 {
            file_system.push(File::File(file_id, c.to_digit(10).unwrap() as usize));
            file_id += 1;
        } else {
            file_system.push(File::Empty(c.to_digit(10).unwrap() as usize));
        }
    });
    file_system
        .into_iter()
        .filter(|file| file.size() != 0)
        .collect()
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
                File::File(0, 1),
                File::Empty(2),
                File::File(1, 3),
                File::Empty(4),
                File::File(2, 5),
            ]
        );
    }

    #[test]
    fn example_checksum_test() {
        let mut example = parse_file_system("2333133121414131402");

        defragment_file_system(&mut example);

        assert_eq!(checksum(&example), 2858);
    }

    #[test]
    fn short_example_test() {
        let mut files = parse_file_system("13254");

        defragment_file_system(&mut files);

        // 0...11.....2222
        // 0...112222.....
        // 011...2222.....
        assert_eq!(
            files,
            vec![
                File::File(0, 1),
                File::File(1, 2),
                File::Empty(3),
                File::File(2, 4),
                File::Empty(5),
            ]
        );
        assert_eq!(
            checksum(&files),
            (1 * 1) + (2 * 1) + (6 * 2) + (7 * 2) + (8 * 2) + (9 * 2)
        );
    }

    #[test]
    fn edgecase_test() {
        // 0..1....22..3333
        // 0..1333322......
        // 02213333........
        let mut files = parse_file_system("1214224");

        defragment_file_system(&mut files);

        assert_eq!(
            files,
            vec![
                File::File(0, 1),
                File::File(2, 2),
                File::File(1, 1),
                File::File(3, 4),
                File::Empty(8)
            ]
        );
    }
}
