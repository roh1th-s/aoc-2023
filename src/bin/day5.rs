use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

type RangeMap = (Range<u64>, Range<u64>);
type AlmanacData = HashMap<String, Vec<RangeMap>>;

fn parse_sections_of_almanac(string: String) -> (Vec<u64>, AlmanacData) {
    let sections = string.split("\n\n").collect::<Vec<&str>>();

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
            let mut lines = section.split("\n");
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

fn get_overlapping(
    test_range: &Range<u64>,
    source_range: &Range<u64>,
) -> (Option<Range<u64>>, Vec<Range<u64>>) {
    let mut overlapping_range = None;
    let mut remaining_ranges = vec![];

    if test_range.start >= source_range.start && test_range.start < source_range.end {
        // SSSSSSS
        //  TTTT
        if test_range.end > source_range.end {
            // SSSSSSS
            //    TTTTT
            overlapping_range = Some(test_range.start..source_range.end);
            remaining_ranges.push(source_range.end..test_range.end);
        } else {
            overlapping_range = Some(test_range.clone());
        }
    } else if test_range.start < source_range.start && test_range.end > source_range.start {
        remaining_ranges.push(test_range.start..source_range.start);
        if test_range.end <= source_range.end {
            //   SSSSSSS
            // TTTTT
            overlapping_range = Some(source_range.start..test_range.end);
        } else if test_range.end > source_range.end {
            //   SSSSSSS
            // TTTTTTTTTTT
            overlapping_range = Some(source_range.clone());
            remaining_ranges.push(source_range.end..test_range.end);
        }
    }

    (overlapping_range, remaining_ranges)
}

fn get_dest_ranges_from_seed_range(seed_range: Range<u64>, almanac_data: &AlmanacData) -> Vec<Range<u64>> {
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

    let mut curr_ranges: Vec<Range<u64>> = [seed_range].to_vec();
    let mut dest_ranges: Vec<Range<u64>> = vec![];

    for map in maps {
        dest_ranges = vec![];

        // create queue of current ranges that need to be mapped to dest_ranges
        let mut curr_range_queue = VecDeque::from(curr_ranges);

        while !curr_range_queue.is_empty() {
            let curr_range = &mut curr_range_queue.pop_front().unwrap();

            let mut found_overlap = false;

            for ranges in map {
                let source_range = &ranges.0;
                let destination_range = &ranges.1;

                // check overlap
                let (overlap, remaining) = get_overlapping(curr_range, source_range);

                if let Some(overlapping_range) = overlap {
                    dest_ranges.push(
                        destination_range.start + overlapping_range.start - source_range.start
                            ..destination_range.start + overlapping_range.end - source_range.start,
                    );

                    // add the remaining ranges to the queue, to map them separately
                    remaining
                        .iter()
                        .for_each(|r| curr_range_queue.push_back(r.clone()));
                    found_overlap = true;
                    break;
                }
            }

            if !found_overlap {
                dest_ranges.push(curr_range.clone());
            }
        }
        curr_ranges = dest_ranges.clone();
    }

    dest_ranges
}

fn part1(seeds: &Vec<u64>, parsed_data: &AlmanacData) -> u64 {
    seeds
        .iter()
        .map(|seed_no| get_dest_ranges_from_seed_range(*seed_no..*seed_no, parsed_data)[0].start)
        .min()
        .unwrap()
}

fn part2(seeds: &Vec<u64>, parsed_data: &AlmanacData) -> u64 {
    seeds
        .windows(2)
        .step_by(2)
        .map(|seed_range| {
            get_dest_ranges_from_seed_range(seed_range[0]..seed_range[0] + seed_range[1], parsed_data)
                .iter()
                .map(|dest_range| dest_range.start)
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
