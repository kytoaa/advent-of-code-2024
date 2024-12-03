#[allow(dead_code)]
pub fn run(mut dir: String) {
    dir.push_str("/day2.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = split_into_levels(&data[..])
        .into_iter()
        .map(|report| report_is_safe(report))
        .filter(|b| *b)
        .count();
    println!("safe reports: {}", result);

    let result = split_into_levels(&data[..])
        .into_iter()
        .map(|report| report_is_safe_with_dampener(report))
        .filter(|b| *b)
        .count();
    println!("safe reports with dampeners: {}", result);
}

fn split_into_levels<'a>(data: &'a str) -> impl Iterator<Item = impl Iterator<Item = u32> + 'a> {
    let reports = data.trim().split('\n');

    reports
        .into_iter()
        .map(|report| report.split_whitespace().map(|n| n.parse().unwrap()))
}

fn report_is_safe_with_dampener<T>(report: T) -> bool
where
    T: Iterator<Item = u32>,
{
    let mut report: Vec<u32> = report.collect();

    if report_is_safe(report.iter().map(|n| *n)) {
        return true;
    }

    for i in 0..report.len() {
        let n = report.remove(i);
        if report_is_safe(report.iter().map(|n| *n)) {
            return true;
        }
        report.insert(i, n);
    }
    false
}

fn report_is_safe<T>(report: T) -> bool
where
    T: Iterator<Item = u32>,
{
    let report: Vec<u32> = report.collect();

    let direction = {
        let dir = report[0].cmp(&report[1]);
        if dir == std::cmp::Ordering::Equal {
            return false;
        };
        dir
    };

    let mut prev = report[0];
    for item in report.iter().skip(1) {
        if prev.cmp(item) != direction {
            return false;
        }
        let diff = prev.abs_diff(*item);
        if diff < 1 || diff > 3 {
            return false;
        }
        prev = *item;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
    ";

    #[test]
    fn level_split_test() {
        let result: Vec<Vec<u32>> = split_into_levels(DATA).map(|i| i.collect()).collect();
        assert_eq!(
            result,
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]
        )
    }

    #[test]
    fn safe_report_test() {
        let result = split_into_levels(DATA)
            .into_iter()
            .map(|report| report_is_safe(report))
            .filter(|b| *b)
            .count();
        assert_eq!(result, 2);
    }

    #[test]
    fn safe_report_with_dampener_test() {
        let result = split_into_levels(DATA)
            .into_iter()
            .map(|report| report_is_safe_with_dampener(report))
            .filter(|b| *b)
            .count();
        assert_eq!(result, 4);
    }
}
