use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

fn count_each_character(string: &String) -> HashMap<char, usize> {
    let mut char_map = HashMap::new();
    for c in string.chars() {
        char_map.insert(
            c,
            if char_map.contains_key(&c) {
                char_map[&c] + 1
            } else {
                1
            },
        );
    }
    char_map
}

fn get_best_hand_with_joker(hand: &String) -> HandType {
    // get best hand by trial and error: replacing J with each of the other cards

    if !hand.contains("J") {
        return get_type_of_hand(hand);
    } else {
        let mut best_hand = get_type_of_hand(hand);
        for card in "AKQT98765432".split("") {
            let hand_type = get_type_of_hand(&hand.replace("J", card));
            if hand_type.partial_cmp(&best_hand).unwrap() == Ordering::Greater {
                best_hand = hand_type;
            }
        }
        return best_hand;
    }
}

fn get_type_of_hand(hand: &String) -> HandType {
    let card_counts = count_each_character(hand);
    let no_of_unique_cards = card_counts.keys().count();

    if no_of_unique_cards == 1 {
        return HandType::FiveOfAKind;
    } else if no_of_unique_cards == 2 {
        if card_counts.values().into_iter().any(|&cnt| cnt == 4) {
            return HandType::FourOfAKind;
        } else {
            return HandType::FullHouse;
        }
    } else if no_of_unique_cards == 3 {
        if card_counts.values().into_iter().any(|&cnt| cnt == 3) {
            return HandType::ThreeOfAKind;
        } else {
            return HandType::TwoPair;
        }
    } else if no_of_unique_cards == 4 {
        return HandType::OnePair;
    } else {
        return HandType::HighCard;
    }
}

fn compare_card(card1: &char, card2: &char, with_joker: bool) -> Ordering {
    if card1 == card2 {
        return Ordering::Equal;
    }
    let cards = if with_joker {
        "AKQT98765432J"
    } else {
        "AKQJT98765432"
    };

    for c in cards.chars() {
        if &c == card1 {
            // found card1 first
            return Ordering::Greater;
        }
        if &c == card2 {
            return Ordering::Less;
        }
    }

    // should never reach this
    return card1.cmp(card2);
}

fn compare_hands(hand1: &String, hand2: &String, with_joker: bool) -> Ordering {
    let hand1_type = if with_joker {
        get_best_hand_with_joker(hand1)
    } else {
        get_type_of_hand(hand1)
    };
    let hand2_type = if with_joker {
        get_best_hand_with_joker(hand2)
    } else {
        get_type_of_hand(hand2)
    };

    if hand1_type == hand2_type {
        for (idx, c) in hand1.char_indices() {
            let comparison = compare_card(&c, &hand2.chars().nth(idx).unwrap(), with_joker);
            if comparison != Ordering::Equal {
                return comparison;
            }
        }
        return Ordering::Equal;
    } else {
        hand1_type.partial_cmp(&hand2_type).unwrap()
    }
}

fn part1(hand_bid_pairs: &Vec<(String, u64)>) -> u64 {
    let mut hand_bid_pairs = hand_bid_pairs.clone();
    hand_bid_pairs.sort_by(|a, b| compare_hands(&a.0, &b.0, false));

    hand_bid_pairs
        .iter()
        .enumerate()
        .map(|(idx, pair)| pair.1 * (idx as u64 + 1))
        .sum()
}

fn part2(hand_bid_pairs: &Vec<(String, u64)>) -> u64 {
    let mut hand_bid_pairs = hand_bid_pairs.clone();
    hand_bid_pairs.sort_by(|a, b| compare_hands(&a.0, &b.0, true));

    hand_bid_pairs
        .iter()
        .enumerate()
        .map(|(idx, pair)| pair.1 * (idx as u64 + 1))
        .sum()
}

pub fn main() {
    let hand_bid_pairs = String::from_utf8(include_bytes!("sample_input_data/day7.txt").to_vec())
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut line_split = line.split_whitespace();

            let hand = line_split.next().unwrap().to_string();
            let bid = line_split.next().unwrap().parse::<u64>().unwrap();

            (hand, bid)
        })
        .collect::<Vec<(String, u64)>>();

    println!("{}", part1(&hand_bid_pairs));
    println!("{}", part2(&hand_bid_pairs));
}
