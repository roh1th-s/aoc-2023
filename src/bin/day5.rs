use std::{collections::HashMap, ops::Range};

type RangeMap = (Range<u64>, Range<u64>);
type AlmanacData = HashMap<String, Vec<RangeMap>>;

fn parse_sections_of_almanac(string: String) -> (Vec<u64>, AlmanacData) {
    let sections = string.split("\r\n\r\n").collect::<Vec<&str>>();

    let mut parsed_data: AlmanacData = HashMap::new();
    let mut seeds = vec![];

    for section in sections {
        if section.starts_with("seeds") {
            seeds = section
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" ")
                .map(|num_str| num_str.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
        } else {
            let mut lines = section.split("\r\n");
            let map_name = lines.nth(0).unwrap().replace("map:", "").trim().to_string(); // removes first line

            let mut range_maps: Vec<RangeMap> = vec![];

            for range_line in lines {
                let nums = range_line
                    .split(" ")
                    .map(|num_str| num_str.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                let destination_range_start = nums[0];
                let source_range_start = nums[1];
                let range_length = nums[2];

                range_maps.push((
                    source_range_start..source_range_start + range_length,
                    destination_range_start..destination_range_start + range_length,
                ))
            }

            parsed_data.insert(map_name.to_string(), range_maps);
        }
    }

    (seeds, parsed_data)
}

fn get_location_no_from_seed(seed_no: u64, almanac_data: &AlmanacData) -> u64 {
    let maps = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ]
    .iter()
    .map(|&map_name| &almanac_data[map_name])
    .collect::<Vec<&Vec<RangeMap>>>();

    let mut val = seed_no;

    for map in maps {
        for ranges in map {
            let source_range = &ranges.0;
            let destination_range = &ranges.1;

            if source_range.contains(&val) {
                let idx = val - source_range.start;
                val = destination_range.start + idx;
                break;
            }
        }
    }

    val
}

fn part1(seeds: &Vec<u64>, parsed_data: &AlmanacData) -> u64 {
    seeds
        .iter()
        .map(|seed_no| get_location_no_from_seed(*seed_no, parsed_data))
        .min()
        .unwrap()
}

fn part2(seeds: &Vec<u64>, parsed_data: &AlmanacData) -> u64 {
    seeds
        .windows(2)
        .step_by(2)
        .map(|seed_range| {
            (seed_range[0]..seed_range[0] + seed_range[1])
                .map(|seed_no| get_location_no_from_seed(seed_no, parsed_data))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

pub fn main() {
    let (seeds, parsed_data) = parse_sections_of_almanac(
        String::from_utf8(include_bytes!("sample_input_data/day5.txt").to_vec()).unwrap(),
    );

    println!("{:?}", part1(&seeds, &parsed_data));
    println!("{:?}", part2(&seeds, &parsed_data));
}
