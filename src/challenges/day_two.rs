use std::str::FromStr;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn execute_solution() {
    println!("Day two challenge can be found in https://adventofcode.com/2023/day/2");
    println!("Solutions:");
    println!();

    let file_str = include_str!("..\\input\\day-two-input.txt");

    let lines = file_str.lines().map(String::from).collect::<Vec<String>>();

    let solutions = find_solutions(lines);

    println!("Sum of possible games: {}", solutions.0);
    println!("Sum of the power of minimum cubes: {}", solutions.1);
}

fn find_solutions(lines: Vec<String>) -> (u32, u32) {
    let mut games_sum: u32 = 0;
    let mut power_sum: u32 = 0;

    for line in lines {
        let split: Vec<&str> = line.split(':').collect();
        let game = u32::from_str(split[0].split_whitespace().collect::<Vec<&str>>()[1]).unwrap();

        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        let combinations = split[1].split(';').collect::<Vec<&str>>();

        let mut possible = true;

        for combination in combinations {
            let cubes = combination.split([' ', ','])
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();

            for i in (0..cubes.len()).step_by(2) {
                if cubes[i + 1] == "red" { reds += u32::from_str(cubes[i]).unwrap() }
                if cubes[i + 1] == "green" { greens += u32::from_str(cubes[i]).unwrap() }
                if cubes[i + 1] == "blue" { blues += u32::from_str(cubes[i]).unwrap() }
            }

            if reds > MAX_RED || greens > MAX_GREEN || blues > MAX_BLUE {
                possible = false;
            }

            if reds > min_red { min_red = reds }
            if greens > min_green { min_green = greens }
            if blues > min_blue { min_blue = blues }

            reds = 0;
            greens = 0;
            blues = 0;
        }

        power_sum += min_red * min_green * min_blue;

        if possible {
            games_sum += game;
        }
    }

    (games_sum, power_sum)
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_two::find_solutions;

    // string inputs provided by AoC examples

    const INPUT_STR: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn should_return_part_one_and_two_correctly() {
        let solutions = find_solutions(INPUT_STR.lines().map(String::from).collect());

        assert_eq!(solutions.0, 8);
        assert_eq!(solutions.1, 2286);
    }
}