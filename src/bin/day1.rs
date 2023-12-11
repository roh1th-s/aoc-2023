fn part1(input_lines: &Vec<String>) -> u32 {
    input_lines
        .iter()
        .map(|line| {
            let d1 = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();

            let d2 = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();

            d1 * 10 + d2
        })
        .sum()
}

fn part2(input_lines: &Vec<String>) -> u32 {
    let num_names = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input_lines
        .iter()
        .map(|line| {
            let mut d1: u32 = 0;
            let mut d2: u32 = 0;

            for (idx, chr) in line.chars().enumerate() {
                match num_names
                    .iter()
                    .position(|&nn| idx + nn.len() < line.len() && &line[idx..idx + nn.len()] == nn)
                {
                    Some(num) => {
                        // if number name found
                        d1 = num as u32;
                        break;
                    }
                    None => {
                        // if no number name found, check if chr is a digit
                        if chr.is_ascii_digit() {
                            d1 = chr.to_digit(10).unwrap();
                            break;
                        }
                    }
                };
            }

            let line_rev = line.chars().rev().collect::<String>();

            for (idx, chr) in line_rev.chars().enumerate() {
                match num_names.iter().position(|&nn| {
                    idx + nn.len() < line_rev.len()
                        && &line_rev[idx..idx + nn.len()] == nn.chars().rev().collect::<String>()
                }) {
                    Some(num) => {
                        // if number name found
                        d2 = num as u32;
                        break;
                    }
                    None => {
                        // if no number name found, check if chr is a digit
                        if chr.is_ascii_digit() {
                            d2 = chr.to_digit(10).unwrap();
                            break;
                        }
                    }
                };
            }

            d1 * 10 + d2
        })
        .sum()
}

pub fn main() {
    let lines = include_bytes!("sample_input_data/day1.txt")
        .split(|c| c == &b'\n')
        .map(|line| String::from_utf8(line.to_vec()).unwrap())
        .collect::<Vec<String>>();

    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}
