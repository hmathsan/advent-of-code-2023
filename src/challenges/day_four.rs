use std::collections::HashMap;

pub fn execute_solution() {
    println!("Day four challenge can be found in https://adventofcode.com/2023/day/4");
    println!("Solutions:");
    println!();

    let file_str = include_str!("..\\input\\day-four-input.txt");

    let lines: Vec<String> = file_str.lines().map(String::from).collect();

    let solutions = find_solutions(lines);

    println!("part one - total points: {}", solutions.0);
    println!("part two - total cards: {}", solutions.1);
}

fn find_solutions(lines: Vec<String>) -> (i32, i32) {
    let lines: Vec<String> = lines.iter()
        .map(|l| String::from(l.split(':').collect::<Vec<&str>>()[1]))
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

    (points, (lines.len() + usize::try_from(additional_cards).unwrap()) as i32)
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_four::find_solutions;

    const INPUT_STR: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn should_return_both_solutions_correctly() {
        let solutions = find_solutions(INPUT_STR.lines().map(String::from).collect());

        assert_eq!(solutions.0, 13);
        assert_eq!(solutions.1, 30);
    }
}