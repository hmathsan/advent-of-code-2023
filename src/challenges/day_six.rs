use std::str::FromStr;
use std::time::Instant;
use std::usize;

pub fn execute_solution() {
    let file_str = include_str!("..\\input\\day-six-input.txt");

    part_one(file_str);
    part_two(file_str);
}

fn part_two(file_str: &str) -> u32 {
    let instant = Instant::now();
    let time = u32::from_str(
        &*file_str
            .lines()
            .nth(0).unwrap()
            .split(':')
            .nth(1).unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
    ).unwrap();

    let distance = usize::from_str(
        &*file_str
            .lines()
            .nth(1).unwrap()
            .split(':')
            .nth(1).unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
    ).unwrap();

    let solution = get_winning_times(&time, &distance);
    println!("Part Two - solution: {}", solution);
    println!("Part One finished in {:.2?}", instant.elapsed());
    solution
}

fn part_one(file_str: &str) -> u32 {
    let instant = Instant::now();

    let time: Vec<u32> = file_str
        .lines()
        .nth(0).unwrap()
        .split(':')
        .nth(1).unwrap()
        .split_whitespace()
        .map(|v| u32::from_str(v).unwrap())
        .collect();

    let distance: Vec<usize> = file_str
        .lines()
        .nth(1).unwrap()
        .split(':')
        .nth(1).unwrap()
        .split_whitespace()
        .map(|v| usize::from_str(v).unwrap())
        .collect();

    let mut game_mul_results = 1;
    for (i, t) in time.iter().enumerate() {
        let d = distance.get(i).unwrap();

        game_mul_results *= get_winning_times(t, d);
    }

    println!("Part One - solution: {}", game_mul_results);
    println!("Part One finished in {:.2?}", instant.elapsed());
    game_mul_results
}

fn get_winning_times(t: &u32, d: &usize) -> u32 {
    let mut first = 0;
    let mut last = 0;

    for i in 0..t.clone() {
        if is_winning_time(t, d, i) {
            first = i;
            break;
        }
    }

    for i in (0..t + 1).rev() {
        if is_winning_time(t, d, i) {
            last = i + 1;
            break;
        }
    }

    last - first
}

fn is_winning_time(t: &u32, d: &usize, hold_time: u32) -> bool {
    let time_left = t - hold_time;

    if time_left <= 0 {
        return false;
    }

    if d < &(time_left as usize * hold_time as usize) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_six::part_one;

    const INPUT:&str = r#"Time:      7  15   30
        Distance:  9  40  200"#;

    #[test]
    fn should_return_correct_part_one() {
        assert_eq!(part_one(INPUT), 288);
    }

    fn should_return_correct_part_two() {
        assert_eq!(part_one(INPUT), 71503);
    }
}