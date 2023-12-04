use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn main() {
    println!("Day two challenge can be found in https://adventofcode.com/2023/day/2");
    println!("Solutions:");
    println!();

    let file_path = "C:\\Users\\mathe\\Documentos\\source\\advent-of-code-2023\\src\\day-two-input.txt";

    let mut games_sum: u32 = 0;
    let mut power_sum: u32 = 0;
    let mut possible_games: Vec<u32> = Vec::new();

    for line in BufReader::new(File::open(file_path).unwrap()).lines().flatten() {
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
            possible_games.push(game);
        }
    }

    println!("Possible games: {:?}", possible_games);
    println!("Sum of possible games: {}", games_sum);
    println!("Sum of the power of minimum cubes: {}", power_sum);
}