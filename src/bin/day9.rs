/// returns an array of histories
fn parse_input(input: &String) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn part1(histories: &Vec<Vec<i64>>) -> i64 {
    histories
        .iter()
        .map(|history| {
            let mut reached_last_sequence = false;
            let mut curr_sequence: Vec<i64> = history.clone();
            let mut last_values: Vec<i64> = vec![curr_sequence.last().unwrap().clone()];

            while !reached_last_sequence {
                curr_sequence = curr_sequence
                    .iter()
                    .enumerate()
                    .map(|(idx, val)| {
                        if idx + 1 < curr_sequence.len() {
                            curr_sequence[idx + 1] - val
                        } else {
                            val.clone()
                        }
                    })
                    .collect();
                curr_sequence.pop(); // remove last element which is 0

                let first = curr_sequence.first().unwrap();
                if curr_sequence.iter().all(|n| n == first) {
                    reached_last_sequence = true;
                }
                last_values.push(curr_sequence.last().unwrap().clone());
            }

            last_values.iter().sum::<i64>()
        })
        .sum()
}

fn part2(histories: &Vec<Vec<i64>>) -> i64 {
    histories
        .iter()
        .map(|history| {
            let mut reached_last_sequence = false;
            let mut curr_sequence: Vec<i64> = history.clone();
            let mut first_values: Vec<i64> = vec![curr_sequence.first().unwrap().clone()];

            while !reached_last_sequence {
                curr_sequence = curr_sequence
                    .iter()
                    .enumerate()
                    .map(|(idx, val)| {
                        if idx + 1 < curr_sequence.len() {
                            curr_sequence[idx + 1] - val
                        } else {
                            val.clone()
                        }
                    })
                    .collect();
                curr_sequence.pop(); // remove last element which is 0

                let first = curr_sequence.first().unwrap();
                if curr_sequence.iter().all(|n| n == first) {
                    reached_last_sequence = true;
                }
                first_values.push(curr_sequence.first().unwrap().clone());
            }
            first_values
                .iter()
                .copied()
                .enumerate()
                .fold(0, |acc, (idx, val)| {
                    acc - val * (if idx % 2 != 0 { 1 } else { -1 }) as i64
                })
        })
        .sum()
}

fn main() {
    let input = String::from_utf8(include_bytes!("sample_input_data/day9.txt").to_vec()).unwrap();
    let histories = parse_input(&input);

    println!("{}", part1(&histories));
    println!("{}", part2(&histories));
}
