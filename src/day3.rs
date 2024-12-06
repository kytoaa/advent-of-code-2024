// this code is very messy D:

#[allow(dead_code)]
pub fn run(mut dir: String) {
    dir.push_str("/day3.txt");
    println!("{}", dir);
    let data = std::fs::read_to_string(dir).unwrap();

    println!("result: {}", solve(&data[..]))
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn solve(data: &str) -> u32 {
    let mut total = 0;

    let mut data = data.chars().peekable();
    while let Some(i) = next_instruction(&mut data) {
        match i {
            Instruction::Dont => {
                while let Some(i) = next_instruction(&mut data) {
                    if i == Instruction::Do {
                        break;
                    }
                }
            }
            Instruction::Mul(l, r) => total += l * r,
            Instruction::Do => (),
        }
    }

    total
}

fn next_instruction(chars: &mut impl Iterator<Item = char>) -> Option<Instruction> {
    let mut chars = chars.peekable();
    loop {
        while let Some(c) = chars.next() {
            if c == 'd' {
                match try_parse_do(&mut chars) {
                    Some(v) => return Some(v),
                    None => (),
                }
            }
            if c == 'm' {
                break;
            }
        }

        match chars.next()? {
            'u' => (),
            _ => continue,
        }
        match chars.next()? {
            'l' => (),
            _ => continue,
        }
        match chars.next()? {
            '(' => (),
            _ => continue,
        }

        let l = match read_num(&mut chars) {
            Some(n) => n,
            None => continue,
        };

        match chars.next()? {
            ',' => (),
            _ => continue,
        }

        let r = match read_num(&mut chars) {
            Some(n) => n,
            None => continue,
        };

        match chars.next()? {
            ')' => (),
            _ => continue,
        }

        return Some(Instruction::Mul(l, r));
    }
}

fn read_num(chars: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Option<u32> {
    let mut buf = String::new();

    while let Some(c) = chars.peek() {
        if buf.len() > 3 || !c.is_numeric() {
            return buf.parse::<u32>().ok();
        }
        buf.push(chars.next()?);
    }

    None
}

fn try_parse_do(chars: &mut impl Iterator<Item = char>) -> Option<Instruction> {
    match chars.next()? {
        'o' => (),
        _ => return None,
    }
    match chars.next()? {
        '(' => (),
        'n' => return try_parse_dont(chars),
        _ => return None,
    }
    match chars.next()? {
        ')' => (),
        _ => return None,
    }
    Some(Instruction::Do)
}
fn try_parse_dont(chars: &mut impl Iterator<Item = char>) -> Option<Instruction> {
    match chars.next()? {
        '\'' => (),
        _ => return None,
    }
    match chars.next()? {
        't' => (),
        _ => return None,
    }
    match chars.next()? {
        '(' => (),
        _ => return None,
    }
    match chars.next()? {
        ')' => (),
        _ => return None,
    }
    Some(Instruction::Dont)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_num_test() {
        let text = String::from("11,");
        assert_eq!(read_num(&mut text.chars().peekable()), Some(11));

        let text = String::from("111;");
        assert_eq!(read_num(&mut text.chars().peekable()), Some(111));

        let text = String::from("1 11;");
        assert_eq!(read_num(&mut text.chars().peekable()), Some(1));

        let text = String::from(" 1");
        assert_eq!(read_num(&mut text.chars().peekable()), None);
    }

    #[test]
    fn get_instruction_test() {
        let text = String::from("mul(3,5)");
        assert_eq!(
            next_instruction(&mut text.chars()),
            Some(Instruction::Mul(3, 5))
        );

        let text = String::from("foomu!mul(3,5)");
        assert_eq!(
            next_instruction(&mut text.chars()),
            Some(Instruction::Mul(3, 5))
        );

        let text = String::from("mul(4*");
        assert_eq!(next_instruction(&mut text.chars()), None);

        let text = String::from("mul(6,9!");
        assert_eq!(next_instruction(&mut text.chars()), None);

        let text = String::from("?(12,34)");
        assert_eq!(next_instruction(&mut text.chars()), None);

        let text = String::from("mul ( 2 , 4 )");
        assert_eq!(next_instruction(&mut text.chars()), None);

        let text = String::from("do() mul( 2 , 4 )");
        assert_eq!(next_instruction(&mut text.chars()), Some(Instruction::Do));

        let text = String::from("don't() mul( 2 , 4 )");
        assert_eq!(next_instruction(&mut text.chars()), Some(Instruction::Dont));
    }

    #[test]
    fn solve_example_test() {
        let text = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(solve(&text[..]), 48);
    }
}
