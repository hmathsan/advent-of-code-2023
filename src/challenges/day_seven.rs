use std::collections::HashMap;
use std::str::FromStr;
use crate::challenges::day_seven::Hand::*;

const CARD_RANK: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARD_RANK_J: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

enum Hand {
    FiveOfKing,
    FourOfKing,
    FullHouse,
    ThreeOfKing,
    TwoPair,
    OnePair,
    HighCard,
    Default,
}

pub fn execute_solution() {
    let file_str = include_str!("..\\input\\day-seven-input.txt");

    println!("Part One Winning Total: {}", part_one(file_str));
    println!("Part Two Winning Total: {}", part_two(file_str));

}

fn part_two(file_str: &str) -> u32 {
    let mut five_of_king: Vec<(String, u32)> = vec![];
    let mut four_of_king: Vec<(String, u32)> = vec![];
    let mut full_house: Vec<(String, u32)> = vec![];
    let mut three_of_king: Vec<(String, u32)> = vec![];
    let mut two_pair: Vec<(String, u32)> = vec![];
    let mut one_pair: Vec<(String, u32)> = vec![];
    let mut high_card: Vec<(String, u32)> = vec![];

    for line in file_str.lines() {
        let hand = line.split_whitespace().nth(0).unwrap().to_string();
        let bid = u32::from_str(line.split_whitespace().nth(1).unwrap()).unwrap();

        let mut unique_hand_chars: HashMap<char, u8> = HashMap::new();

        for (i, ch) in hand.chars().enumerate() {
            let mut char_to_add = ch;
            if char_to_add == 'J' {
                let mut previous_char: Option<char> = None;
                let mut previous_char_count: u8 = 0;

                let mut next_char: Option<char> = None;
                let mut next_char_count: u8 = 0;

                for previous_char_i in (0..i).rev() {
                    if let Some(pc) = hand.chars().nth(previous_char_i) {
                        if pc != 'J' {
                            previous_char = Some(pc);
                            previous_char_count = hand
                                .chars().filter(|c| c == &pc).count() as u8;
                            break;
                        }
                    }
                }

                for next_char_i in i..hand.len() {
                    if let Some(nc) = hand.chars().nth(next_char_i) {
                        if nc != 'J' {
                            next_char = Some(nc);
                            next_char_count = hand
                                .chars().filter(|c| c == &nc).count() as u8;
                            break;
                        }
                    }
                }

                if let Some(pc) = previous_char {
                    char_to_add = pc;
                } else {
                    if let Some(nc) = next_char {
                        char_to_add = nc;
                    }
                }
            }

            if let Some(v) = unique_hand_chars.get(&char_to_add) {
                unique_hand_chars.insert(char_to_add, v + 1);
            } else {
                unique_hand_chars.insert(char_to_add, 1);
            }
        }

        match get_hand_type(unique_hand_chars) {
            FiveOfKing => five_of_king.push((hand.clone(), bid)),
            FourOfKing => four_of_king.push((hand.clone(), bid)),
            FullHouse => full_house.push((hand.clone(), bid)),
            ThreeOfKing => three_of_king.push((hand.clone(), bid)),
            TwoPair => two_pair.push((hand.clone(), bid)),
            OnePair => one_pair.push((hand.clone(), bid)),
            HighCard => high_card.push((hand.clone(), bid)),
            Default => continue
        }
    }

    let mut rank: Vec<(String, u32)> = vec![];
    rank.extend(sort_ranks(high_card, 0, CARD_RANK_J));

    rank.extend(sort_ranks(one_pair, 0, CARD_RANK_J));

    rank.extend(sort_ranks(two_pair, 0, CARD_RANK_J));

    rank.extend(sort_ranks(three_of_king, 0, CARD_RANK_J));

    rank.extend(sort_ranks(full_house, 0, CARD_RANK_J));

    rank.extend(sort_ranks(four_of_king, 0, CARD_RANK_J));

    rank.extend(sort_ranks(five_of_king, 0, CARD_RANK_J));

    let mut total_winning = 0;

    for (i, (_card, bid)) in rank.iter().enumerate() {
        total_winning += bid * (i as u32 + 1);
    }

    total_winning
}

fn part_one(file_str: &str) -> u32 {
    let mut five_of_king: Vec<(String, u32)> = vec![];
    let mut four_of_king: Vec<(String, u32)> = vec![];
    let mut full_house: Vec<(String, u32)> = vec![];
    let mut three_of_king: Vec<(String, u32)> = vec![];
    let mut two_pair: Vec<(String, u32)> = vec![];
    let mut one_pair: Vec<(String, u32)> = vec![];
    let mut high_card: Vec<(String, u32)> = vec![];

    for line in file_str.lines() {
        let hand = line.split_whitespace().nth(0).unwrap().to_string();
        let bid = u32::from_str(line.split_whitespace().nth(1).unwrap()).unwrap();

        let mut unique_hand_chars: HashMap<char, u8> = HashMap::new();

        for ch in hand.chars() {
            if let Some(v) = unique_hand_chars.get(&ch) {
                unique_hand_chars.insert(ch, v + 1);
            } else {
                unique_hand_chars.insert(ch, 1);
            }
        }

        match get_hand_type(unique_hand_chars) {
            FiveOfKing => five_of_king.push((hand.clone(), bid)),
            FourOfKing => four_of_king.push((hand.clone(), bid)),
            FullHouse => full_house.push((hand.clone(), bid)),
            ThreeOfKing => three_of_king.push((hand.clone(), bid)),
            TwoPair => two_pair.push((hand.clone(), bid)),
            OnePair => one_pair.push((hand.clone(), bid)),
            HighCard => high_card.push((hand.clone(), bid)),
            Default => continue
        }
    }

    let mut rank: Vec<(String, u32)> = vec![];
    rank.extend(sort_ranks(high_card, 0, CARD_RANK));

    rank.extend(sort_ranks(one_pair, 0, CARD_RANK));

    rank.extend(sort_ranks(two_pair, 0, CARD_RANK));

    rank.extend(sort_ranks(three_of_king, 0, CARD_RANK));

    rank.extend(sort_ranks(full_house, 0, CARD_RANK));

    rank.extend(sort_ranks(four_of_king, 0, CARD_RANK));

    rank.extend(sort_ranks(five_of_king, 0, CARD_RANK));

    let mut total_winning = 0;

    for (i, (_card, bid)) in rank.iter().enumerate() {
        total_winning += bid * (i as u32 + 1);
    }

    total_winning
}

fn get_hand_type(unique_hand_chars: HashMap<char, u8>) -> Hand {
    return match unique_hand_chars.len() {
        1 => FiveOfKing,
        2 => {
            if unique_hand_chars.iter().filter(|(k, v)| v == &&2).count() >= 1 {
                FullHouse
            } else {
                FourOfKing
            }
        }
        3 => {
            if unique_hand_chars.iter().filter(|(k, v)| v == &&2).count() >= 2 {
                TwoPair
            } else {
                ThreeOfKing
            }
        }
        4 => OnePair,
        5 => HighCard,
        _ => Default
    };
}

fn sort_ranks(
    vec_to_sort: Vec<(String, u32)>,
    card_i: usize,
    card_ranking: [char; 13],
) -> Vec<(String, u32)> {
    let mut sorted_rank = vec![];

    for card in card_ranking.iter().rev() {
        let hands: Vec<(String, u32)> = vec_to_sort
            .iter()
            .filter(|(c, _)| c.chars().nth(card_i).unwrap() == card.clone())
            .map(|v| v.clone())
            .collect();

        if hands.len() > 1 {
            sort_ranks(hands, card_i + 1, card_ranking).iter().for_each(|v| {
                sorted_rank.push(v.clone())
            })
        } else {
            hands.iter().for_each(|v| {
                sorted_rank.push(v.clone())
            })
        }
    }

    sorted_rank
}

#[cfg(test)]
mod tests {
    use crate::challenges::day_seven::{part_one, part_two};

    const INPUT: &str = r#"2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41
"#;

    #[test]
    fn should_return_part_one_correctly() {
        assert_eq!(part_one(INPUT), 6592);
    }

    #[test]
    fn should_return_part_two_correctly() {
        assert_eq!(part_two(INPUT), 6839);
    }
}