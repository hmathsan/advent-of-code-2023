use std::str::FromStr;

pub fn execute_solution() {
    println!("Day one challenge can be found in https://adventofcode.com/2023/day/1");
    println!("Solutions:");
    println!();

    let file_str = include_str!("..\\input\\day-one-input.txt");

    let lines = file_str.lines().map(String::from);
    println!("Day one - Part one: {}", part_one(lines.clone().collect()));
    println!("Day one - part two: {}", part_two(lines.clone().collect()));
}

struct NumberFromString;

impl NumberFromString {
    pub fn potential_number(bytes: &[u8]) -> Option<u8> {
        let one = [b'o', b'n', b'e'];
        let two = [b't', b'w', b'o'];
        let three = [b't', b'h', b'r', b'e', b'e'];
        let four = [b'f', b'o', b'u', b'r'];
        let five = [b'f', b'i', b'v', b'e'];
        let six = [b's', b'i', b'x'];
        let seven = [b's', b'e', b'v', b'e', b'n'];
        let eight = [b'e', b'i', b'g', b'h', b't'];
        let nine = [b'n', b'i', b'n', b'e'];

        if bytes.len() >= 3 {
            let bytes_to_check = &bytes[0..3];
            if bytes_to_check == one { return Some(b'1'); }
            if bytes_to_check == two { return Some(b'2'); }
            if bytes_to_check == six { return Some(b'6'); }

            if bytes.len() >= 4 {
                let bytes_to_check = &bytes[0..4];
                if bytes_to_check == four { return Some(b'4'); }
                if bytes_to_check == five { return Some(b'5'); }
                if bytes_to_check == nine { return Some(b'9'); }

                if bytes.len() >= 5 {
                    let bytes_to_check = &bytes[0..5];
                    if bytes_to_check == three { return Some(b'3'); }
                    if bytes_to_check == seven { return Some(b'7'); }
                    if bytes_to_check == eight { return Some(b'8'); }
                }
            }
        }

        None
    }
}

fn part_one(lines: Vec<String>) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();

    for line in lines {
        let digits: Vec<u8> = line.as_bytes().iter()
            .filter(|f| f.is_ascii_digit()).copied()
            .collect();

        numbers.push(get_first_and_last_digits(digits));
    }

    numbers.into_iter().sum::<i32>()
}

fn part_two(lines: Vec<String>) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();

    for line in lines {
        let line_bytes = line.as_bytes();

        let digits: Vec<u8> = line.as_bytes().iter().enumerate().map(|(i, b)| {
            if b.is_ascii_digit() {
                return line_bytes[i];
            }

            if let Some(digit) = NumberFromString::potential_number(&line_bytes[i..line_bytes.len()]) {
                return digit;
            }

            b' '
        }).filter(|b| b != &b' ').collect();

        numbers.push(get_first_and_last_digits(digits));
    }

    numbers.into_iter().sum::<i32>()
}

fn get_first_and_last_digits(digits: Vec<u8>) -> i32 {
    let first_digit = String::from_utf8(Vec::from(&[*digits.first().unwrap()])).unwrap();
    let last_digit = if digits.len() < 2 {
        first_digit.clone()
    } else {
        String::from_utf8(Vec::from(&[*digits.last().unwrap()])).unwrap()
    };

    i32::from_str(&format!("{}{}", first_digit, last_digit)[..]).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_one::{part_one, part_two};

    // string inputs provided by AoC examples

    const PART_ONE_INPUT_STR: &str = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;

    const PART_TWO_INPUT_STR: &str = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;

    #[test]
    fn should_return_part_one_correctly() {
        assert_eq!(part_one(PART_ONE_INPUT_STR.lines().map(String::from).collect()), 142)
    }

    #[test]
    fn should_return_part_two_correctly() {
        assert_eq!(part_two(PART_TWO_INPUT_STR.lines().map(String::from).collect()), 281);
    }
}