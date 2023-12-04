use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    println!("Day four challenge can be found in https://adventofcode.com/2023/day/4");
    println!("Solutions:");
    println!();

    let file_path = "C:\\Users\\mathe\\Documentos\\source\\advent-of-code-2023\\src\\day-four-input.txt";

    let lines: Vec<String> = BufReader::new(File::open(file_path).unwrap())
        .lines().flatten()
        .map(|l| String::from(l.clone().split(':').collect::<Vec<&str>>()[1]))
        .collect();

    let mut points = 0;

    let mut cards_copy: HashMap<usize, u32> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let vec = line.split('|').collect::<Vec<&str>>();

        let winning_numbers: Vec<&str> = vec[0].split_whitespace().collect();
        let owned_numbers: Vec<&str> = vec[1].split_whitespace().collect();

        let mut winning_number_count = 0;
        let mut card_points = 0;

        winning_numbers.iter().for_each(|n| {
            if owned_numbers.contains(n) {
                if winning_number_count > 0 {
                    card_points *= 2;
                } else {
                    card_points += 1;
                }

                winning_number_count += 1;
            }
        });

        let current_card = i + 1usize;
        let mut current_card_copies = 0;
        if let Some(card) = cards_copy.get(&(current_card)) {
            current_card_copies = *card;
        }

        for j in 0..winning_number_count {
            let copied_card = usize::try_from(j + 1).unwrap();

            if let Some(card) = cards_copy.get(&(current_card + copied_card)) {
                cards_copy.insert(current_card + copied_card, card + current_card_copies + 1);
            } else {
                cards_copy.insert(current_card + copied_card, current_card_copies + 1);
            }
        }

        points += card_points;
    }

    let mut additional_cards = 0;
    cards_copy.iter().for_each(|(_k, v)| additional_cards += v);

    println!("part one - total points: {}", points);
    println!("part two - total cards: {}", lines.len() + usize::try_from(additional_cards).unwrap());
}