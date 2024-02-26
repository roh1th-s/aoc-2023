fn count_winning_numbers(card_string: &String) -> u32 {
    let number_part = card_string.split(":").nth(1).unwrap().trim();
    let number_part_split = number_part
        .split("|")
        .map(|part| part.trim())
        .collect::<Vec<&str>>();

    let winning_numbers = number_part_split[0]
        .split_whitespace()
        .map(|num_str| num_str.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let numbers_possessed = number_part_split[1]
        .split_whitespace()
        .map(|num_str| num_str.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    numbers_possessed
        .iter()
        .filter(|num| winning_numbers.contains(num))
        .count() as u32
}

fn part1(card_lines: &Vec<String>) -> u64 {
    card_lines
        .iter()
        .map(|card_line| {
            let num_winning_numbers_possessed = count_winning_numbers(&card_line);
            if num_winning_numbers_possessed > 0 {
                2_u64.pow(num_winning_numbers_possessed - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part2(card_lines: &Vec<String>) -> u64 {
    let mut num_scratch_cards_of_id: Vec<u64> = vec![0; card_lines.len()];

    for (idx, card_line) in card_lines.iter().enumerate() {
        let num_winning_numbers = count_winning_numbers(&card_line);

        let copies_of_this_card = num_scratch_cards_of_id[idx] + 1;
        num_scratch_cards_of_id[idx] += 1;

        for card_idx in idx + 1..idx + num_winning_numbers as usize + 1 {
            num_scratch_cards_of_id[card_idx] += copies_of_this_card;
        }
    }

    num_scratch_cards_of_id.iter().sum()
}

pub fn main() {
    let card_lines = String::from_utf8(include_bytes!("sample_input_data/day4.txt").to_vec())
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    println!("Part 1: {}", part1(&card_lines));
    println!("Part 2: {}", part2(&card_lines));
}
