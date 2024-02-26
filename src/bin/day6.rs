struct Race {
    time: u64,
    distance: u64,
}

fn find_no_of_ways_to_beat_race(race: &Race) -> u64 {
    /* 
        solving inequality n(t - n) > d for n, we get the upper and lower limits
        of how long we can hold down button
    */
    let t = race.time as f64;
    let d = race.distance as f64;
    let discriminant = (t * t - 4.0 * d).sqrt();

    let lower_lim = ((t - discriminant) / 2.0).ceil() as u64;
    let upper_lim = ((t + discriminant) / 2.0).floor() as u64;

    return upper_lim - lower_lim + 1;
}

fn parse_kerned_input(lines: &Vec<String>) -> Race {
    let mut race = Race {
        time: 0,
        distance: 0,
    };

    for line in lines {
        let mut line_split = line.split(":");
        let label = line_split.nth(0).unwrap();
        let value = line_split
            .nth(0)
            .unwrap()
            .split_whitespace()
            .map(|n| n.trim())
            .fold(String::new(), |a, b| a + b)
            .parse::<u64>()
            .unwrap();

        if label == "Time" {
            race.time = value
        } else {
            race.distance = value;
        }
    }

    race
}

fn parse_races(lines: &Vec<String>) -> Vec<Race> {
    let mut times = vec![];
    let mut distances = vec![];

    for line in lines {
        let mut line_split = line.split(":");
        let label = line_split.nth(0).unwrap();
        let values = line_split
            .nth(0)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        if label == "Time" {
            times = values;
        } else {
            distances = values;
        }
    }

    let mut races = Vec::new();

    for (idx, &time) in times.iter().enumerate() {
        let d = distances[idx];
        races.push(Race { time, distance: d })
    }

    races
}

fn part1(races: &Vec<Race>) -> u64 {
    races
        .iter()
        .map(|race| find_no_of_ways_to_beat_race(race))
        .product()
}

fn part2(race: &Race) -> u64 {
    find_no_of_ways_to_beat_race(race)
}

pub fn main() {
    let lines = String::from_utf8(include_bytes!("sample_input_data/day6.txt").to_vec())
        .unwrap()
        .split("\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let races = parse_races(&lines);
    println!("part 1: {}", part1(&races));

    let race = parse_kerned_input(&lines);
    println!("part 2: {}", part2(&race));
}
