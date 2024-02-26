use std::cmp;

#[derive(Debug, Clone)]
struct NumSearchResult {
    value: u32,
    start_idx: usize,
    end_idx: usize,
}

/// returns a list of indices of gears
fn parse_gears(string: &str) -> Vec<usize> {
    let mut gear_indices: Vec<usize> = Vec::new();
    for (idx, chr) in string.char_indices() {
        if chr == '*' {
            gear_indices.push(idx);
        }
    }
    gear_indices
}

fn string_has_symbol(string: &str) -> bool {
    // check if string has any character that is neither a digit, nor a '.' (period)
    string
        .chars()
        .any(|c| !(c.is_ascii_digit() || c == char::from(b'.')))
}

/// parse integers in string
fn parse_numbers(string: &String) -> Vec<NumSearchResult> {
    let chars = string.chars().collect::<Vec<char>>();

    let mut results: Vec<NumSearchResult> = Vec::new();

    let mut start_pos = 0;
    let mut end_pos = 0;

    while end_pos < string.len() {
        if chars[start_pos].is_ascii_digit() {
            end_pos = start_pos + 1;

            while end_pos < string.len() && chars[end_pos].is_ascii_digit() {
                end_pos += 1;
            }

            results.push(NumSearchResult {
                value: string[start_pos..end_pos].parse::<u32>().unwrap(),
                start_idx: start_pos,
                end_idx: end_pos - 1,
            });
            start_pos = end_pos
        } else {
            start_pos += 1;
            end_pos += 1;
        }
    }

    results
}

fn parse_part_nums(schematic_lines: &Vec<String>) -> Vec<Vec<NumSearchResult>> {
    schematic_lines
        .iter()
        .enumerate()
        .map(|(idx, schematic_line)| {
            parse_numbers(&schematic_line)
                .iter()
                .filter(|&part_num| {
                    let start_index = cmp::max(part_num.start_idx as i32 - 1, 0) as usize;
                    let end_index = cmp::min(part_num.end_idx + 2, schematic_line.len());

                    if idx > 0 {
                        // there is a line above
                        let line_above = &schematic_lines[idx - 1];

                        if string_has_symbol(&line_above[start_index..end_index]) {
                            return true;
                        }
                    }

                    if idx < schematic_lines.len() - 1 {
                        // if there is a line  below
                        let line_below = &schematic_lines[idx + 1];

                        if string_has_symbol(&line_below[start_index..end_index]) {
                            return true;
                        }
                    }

                    if string_has_symbol(&schematic_line[start_index..start_index + 1])
                        || string_has_symbol(&schematic_line[end_index - 1..end_index])
                    {
                        return true;
                    }
                    false
                })
                .map(|res| res.clone())
                .collect()
        })
        .collect::<Vec<Vec<NumSearchResult>>>()
}

fn part1(schematic_lines: &Vec<String>) -> u64 {
    parse_part_nums(schematic_lines)
        .iter()
        .flat_map(|line_nums| line_nums.iter())
        .map(|res| res.value as u64)
        .sum()
}

fn part2(schematic_lines: &Vec<String>) -> u64 {
    let part_nums_line_wise = parse_part_nums(schematic_lines);

    let mut gear_ratio_sum: u64 = 0;

    for (idx, schematic_line) in schematic_lines.iter().enumerate() {
        // find potential gears in each line
        let potential_gear_indices = parse_gears(&schematic_line);

        for potential_gear_index in potential_gear_indices {
            // for each potential gear, check for adjacent part numbers
            let mut adjacent_part_nums: Vec<u32> = Vec::new();

            if idx > 0 {
                // there is a line above
                let part_nums_above = &part_nums_line_wise[idx - 1];

                for part_num in part_nums_above {
                    if adjacent_part_nums.len() >= 2 {
                        break;
                    }
                    if (part_num.start_idx as i32 - 1..part_num.end_idx as i32 + 2)
                        .contains(&(potential_gear_index as i32))
                    {
                        adjacent_part_nums.push(part_num.value);
                    }
                }
            }

            if idx < schematic_line.len() - 1 {
                // there is line below

                let part_nums_below = &part_nums_line_wise[idx + 1];

                for part_num in part_nums_below {
                    if adjacent_part_nums.len() >= 2 {
                        break;
                    }
                    if (part_num.start_idx as i32 - 1..part_num.end_idx as i32 + 2)
                        .contains(&(potential_gear_index as i32))
                    {
                        adjacent_part_nums.push(part_num.value);
                    }
                }
            }

            // adjacent part numbers on the same line as the gear
            let part_nums_on_current_line = &part_nums_line_wise[idx];
            for part_num in part_nums_on_current_line {
                if adjacent_part_nums.len() >= 2 {
                    break;
                }
                if potential_gear_index as i32 == (part_num.start_idx as i32) - 1
                    || potential_gear_index == part_num.end_idx + 1
                {
                    adjacent_part_nums.push(part_num.value);
                }
            }

            if adjacent_part_nums.len() >= 2 {
                // if there are 2 adjacent part numbers, add their product to overall gear ratio sum
                gear_ratio_sum += (adjacent_part_nums[0] * adjacent_part_nums[1]) as u64;
            }
        }
    }

    gear_ratio_sum
}

pub fn main() {
    let schematic_lines = String::from_utf8(include_bytes!("sample_input_data/day3.txt").to_vec())
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    println!("Part 1: {}", part1(&schematic_lines));
    println!("Part 2: {}", part2(&schematic_lines));
}
