fn part1(games: &Vec<String>) -> u32 {
    games
        .iter()
        .enumerate()
        .map(|(idx, game)| {
            let game_id = idx + 1;
            let set_data = &game[game.find(":").unwrap() + 1..];
            let sets = set_data.split(";").map(|s| s.trim());

            for set in sets {
                let ball_counts = set.split(",").map(|s| s.trim());

                for ball_count in ball_counts {
                    let ball_count_split = ball_count.split(" ").collect::<Vec<&str>>();
                    let count = ball_count_split[0].parse::<u32>().unwrap();
                    let color = ball_count_split[1];

                    if count
                        > match color {
                            "red" => 12,
                            "green" => 13,
                            "blue" => 14,
                            _ => panic!("Error parsing ball color: {}", color),
                        }
                    {
                        return 0;
                    }
                }
            }

            game_id as u32
        })
        .sum()
}

fn part2(games: &Vec<String>) -> u32 {
    games
        .iter()
        .enumerate()
        .map(|(_, game)| {
            let set_data = &game[game.find(":").unwrap() + 1..];
            let sets = set_data.split(";").map(|s| s.trim());

            let mut min_required = vec![0, 0, 0]; // R, G, B

            for set in sets {
                let ball_counts = set.split(",").map(|s| s.trim());

                for ball_count in ball_counts {
                    let ball_count_split = ball_count.split(" ").collect::<Vec<&str>>();
                    let count = ball_count_split[0].parse::<u32>().unwrap();
                    let color = ball_count_split[1];

                    let clr_idx = match color {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => panic!("Error parsing ball color: {}", color),
                    };

                    if count > min_required[clr_idx] {
                        min_required[clr_idx] = count;
                    }
                }
            }
            min_required.iter().product::<u32>()
        })
        .sum()
}

pub fn main() {
    let games = include_bytes!("sample_input_data/day2.txt")
        .split(|c| c == &b'\n')
        .map(|line| String::from_utf8(line.to_vec()).unwrap())
        .collect::<Vec<String>>();

    println!("part 1: {}", part1(&games));
    println!("part 2: {}", part2(&games));
}
