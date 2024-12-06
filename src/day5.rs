#[allow(dead_code)]
pub fn run(mut dir: String) {
    dir.push_str("/day5.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    let result = solve(&data[..]);
    println!("{}", result);
}

fn solve(data: &str) -> u32 {
    let rules = parse_rules(data);
    let updates = parse_updates(data);

    updates
        .into_iter()
        .filter_map(|update| test_update(&update, &rules))
        .sum()
}

type Page = u32;

struct Rule(Page, Page);
type Update = Vec<Page>;

fn test_update(update: &Update, rules: &Vec<Rule>) -> Option<u32> {
    _ = in_order(update, rules)?;

    let mut update = update.clone();
    loop {
        let bad_rule = match in_order(&update, rules) {
            Some(r) => r,
            None => return Some(update[update.len() / 2]),
        };

        let (a, b) = (
            update
                .iter()
                .enumerate()
                .find(|(_, n)| **n == bad_rule.1)
                .unwrap()
                .0,
            update
                .iter()
                .enumerate()
                .find(|(_, n)| **n == bad_rule.0)
                .unwrap()
                .0,
        );
        update.swap(a, b);
    }
}
fn in_order<'a>(update: &Update, rules: &'a Vec<Rule>) -> Option<&'a Rule> {
    let rules: Vec<&Rule> = rules
        .into_iter()
        .filter(|rule| update.contains(&rule.0) && update.contains(&rule.1))
        .collect();

    for rule in &rules {
        if update
            .iter()
            .enumerate()
            .find(|(_, n)| **n == rule.0)
            .unwrap()
            .0
            > update
                .iter()
                .enumerate()
                .find(|(_, n)| **n == rule.1)
                .unwrap()
                .0
        {
            return Some(rule);
        }
    }

    None
}

#[allow(dead_code)]
fn fix_update(mut update: Update, bad_rule: &Rule) -> u32 {
    let num0_index = update
        .iter()
        .enumerate()
        .find(|(_, n)| **n == bad_rule.0)
        .unwrap()
        .0;

    _ = update.remove(
        update
            .iter()
            .enumerate()
            .find(|(_, n)| **n == bad_rule.1)
            .unwrap()
            .0,
    );

    for i in num0_index..update.len() {
        update.insert(i, bad_rule.1);

        if update
            .iter()
            .enumerate()
            .find(|(_, n)| **n == bad_rule.0)
            .unwrap()
            .0
            > update
                .iter()
                .enumerate()
                .find(|(_, n)| **n == bad_rule.1)
                .unwrap()
                .0
        {
            update.remove(i);
        } else {
            return update[update.len() / 2];
        }
    }

    panic!();
}

fn parse_rules(data: &str) -> Vec<Rule> {
    let data = data
        .trim()
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    let mut rules = vec![];

    for line in data {
        if line.is_empty() {
            break;
        }
        let x = parse_number(line);
        assert!(&line[2..3] == "|");
        let y = parse_number(&line[3..5]);

        rules.push(Rule(x, y));
    }

    rules
}
fn parse_number(data: &str) -> Page {
    let num = &data[..2];
    num.parse().unwrap()
}

fn parse_updates(data: &str) -> Vec<Update> {
    let data = data
        .trim()
        .split('\n')
        .map(|line| line.trim())
        .rev()
        .collect::<Vec<&str>>();

    let mut updates = vec![];

    for line in data {
        if line.is_empty() {
            break;
        }
        updates.push(line.split(',').map(|n| n.trim().parse().unwrap()).collect());
    }

    updates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let data = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
        ";

        assert_eq!(solve(data), 123);
    }
}
