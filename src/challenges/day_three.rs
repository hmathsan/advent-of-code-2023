#![allow(clippy::ptr_arg)]

use std::str::FromStr;

pub fn execute_solution() {
    println!("Day three challenge can be found in https://adventofcode.com/2023/day/3");
    println!("Solutions:");
    println!();

    let file_str = include_str!("..\\input\\day-three-input.txt");

    let lines = file_str.lines().map(String::from).collect::<Vec<String>>();

    println!("Sum of part numbers: {}", part_one(lines.clone()));
    println!("Sum of gears: {}", part_two(lines));
}

fn part_two(lines: Vec<String>) -> u32 {
    let mut gears: u32 = 0;

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch != '*' { continue; }

            let chars: Vec<char> = line.chars().collect();

            let mut gear_value = 1;
            let mut gears_found = 0;

            if let Some(value) = check_previous_chars(&chars, j) {
                gears_found += 1;
                gear_value *= value;
            }

            if let Some(value) = check_next_chars(&chars, j) {
                gears_found += 1;
                gear_value *= value;
            }

            if i > 0 {
                let previous_line_chars: Vec<char> = lines[i - 1].chars().collect();

                let line_results = check_other_line(previous_line_chars, j);
                gears_found += line_results.0;
                gear_value *= line_results.1;
            }

            if i < lines.len() - 1 {
                let next_line: Vec<char> = lines[i + 1].chars().collect();

                let line_results = check_other_line(next_line, j);
                gears_found += line_results.0;
                gear_value *= line_results.1;
            }

            if gears_found != 2 {
                continue;
            } else {
                gears += gear_value;
            }
        }
    }

    gears
}

fn part_one(lines: Vec<String>) -> u32 {
    let mut sum_of_part_numbers = 0;
    let mut previous_indexes: Vec<u32> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if previous_indexes.contains(&u32::try_from(j).unwrap()) { continue; }
            if !ch.is_ascii_digit() { continue; }

            let chars: Vec<char> = line.chars().collect();

            let part_number_and_indexes = get_part_number(chars.clone(), j);

            let part_number = part_number_and_indexes.0;
            previous_indexes = part_number_and_indexes.1;

            if is_valid_to_sum(chars, &previous_indexes, false) {
                sum_of_part_numbers += part_number;
                continue;
            }

            if i > 0 {
                let previous_line: Vec<char> = lines[i - 1].clone().chars().collect();

                if is_valid_to_sum(previous_line, &previous_indexes, true) {
                    sum_of_part_numbers += part_number;
                    continue;
                }
            }

            if i < lines.len() - 1 {
                let next_line: Vec<char> = lines[i + 1].clone().chars().collect();

                if is_valid_to_sum(next_line, &previous_indexes, true) {
                    sum_of_part_numbers += part_number;
                    continue;
                }
            }
        }
    }

    sum_of_part_numbers
}

fn is_valid_to_sum(line_to_check: Vec<char>, indexes_to_validate: &Vec<u32>, is_other_line: bool) -> bool {
    let first = usize::try_from(*indexes_to_validate.first().unwrap()).unwrap();
    let last = usize::try_from(*indexes_to_validate.last().unwrap() + 1).unwrap();

    if is_other_line {
        for i in indexes_to_validate {
            let ch = line_to_check[usize::try_from(*i).unwrap()];

            if !ch.is_ascii_digit() && ch != '.' {
                return true;
            }
        }
    }

    for i in first..last {
        if i > 0 {
            let previous_char = line_to_check[i - 1];

            if !previous_char.is_ascii_digit() && previous_char != '.' {
                return true;
            }
        }

        if i < line_to_check.len() - 1 {
            let next_char: char = line_to_check[i + 1];

            if !next_char.is_ascii_digit() && next_char != '.' {
                return true;
            }
        }
    }

    false
}

fn check_previous_chars(chars: &Vec<char>, j: usize) -> Option<u32> {
    if j > 0 && chars[j - 1].is_ascii_digit() {
        return Some(get_part_number(chars.clone(), j - 1).0);
    }

    None
}

fn check_next_chars(chars: &Vec<char>, j: usize) -> Option<u32> {
    if j < chars.len() - 1 && chars[j + 1].is_ascii_digit() {
        return Some(get_part_number(chars.clone(), j + 1).0);
    }

    None
}

fn get_part_number(chars: Vec<char>, j: usize) -> (u32, Vec<u32>) {
    let mut indexes: Vec<u32> = Vec::new();
    indexes.push(u32::try_from(j).unwrap());

    let mut ch: String = String::from(chars[j]);

    for i in (0..j).rev() {
        if chars[i].is_ascii_digit() {
            indexes.push(u32::try_from(i).unwrap());
            ch = format!("{}{}", chars[i], ch);
        } else {
            break;
        }
    }

    for (i, _) in chars.iter().enumerate().skip(j + 1) {
        if chars[i].is_ascii_digit() {
            indexes.push(u32::try_from(i).unwrap());
            ch = format!("{}{}", ch, chars[i]);
        } else {
            break;
        }
    }

    (u32::from_str(&ch[..]).unwrap(), indexes)
}

fn check_other_line(line: Vec<char>, j: usize) -> (u32, u32) {
    let mut gears_found = 0;
    let mut line_total = 1;

    if line[j].is_ascii_digit() {
        gears_found += 1;
        line_total *= get_part_number(line, j).0
    } else {
        if let Some(value) = check_previous_chars(&line, j) {
            gears_found += 1;
            line_total *= value;
        }

        if let Some(value) = check_next_chars(&line, j) {
            gears_found += 1;
            line_total *= value;
        }
    }

    (gears_found, line_total)
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_three::{part_one, part_two};

    const INPUT_STR: &str = r#"12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56"#;

    // Input from Reddit(https://www.reddit.com/r/adventofcode/comments/189q9wv/2023_day_3_another_sample_grid_to_use/)
    // covers a lot more test cases than the example from AoC

    #[test]
    fn should_return_part_one_correctly() {
        assert_eq!(part_one(INPUT_STR.lines().map(String::from).collect()), 925);
    }

    #[test]
    fn should_return_part_two_correctly() {
        assert_eq!(part_two(INPUT_STR.lines().map(String::from).collect()), 6756);
    }
}