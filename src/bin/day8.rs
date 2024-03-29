use std::collections::HashMap;

type NodeMap = HashMap<String, (String, String)>;

fn parse_input(input: &String) -> (Vec<char>, NodeMap) {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = sections[0];
    let instructions_arr = instructions.chars().collect::<Vec<char>>();
    let nodes = sections[1];

    let mut node_map = HashMap::new();

    for node_line in nodes.split("\n") {
        let node_line_split: Vec<&str> = node_line.split(" = ").collect();
        let label = node_line_split[0].to_string();
        let mut elems = node_line_split[1];
        elems = &elems[1..elems.len() - 1];

        let elem_split: Vec<&str> = elems.split(",").map(|e| e.trim()).collect();

        node_map.insert(
            label,
            (elem_split[0].to_string(), elem_split[1].to_string()),
        );
    }

    (instructions_arr, node_map)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(arr: &Vec<u64>) -> u64 {
    let mut result = arr[0];
    for i in 1..arr.len() {
        result = (result * arr[i]) / gcd(result, arr[i]);
    }
    result
}

fn part1(instructions: &Vec<char>, node_map: &NodeMap) -> u64 {
    let instructions_len = instructions.len();
    let mut instruction_idx = 0;
    let mut curr_node = "AAA";
    let mut n_steps = 0;

    while curr_node != "ZZZ" {
        instruction_idx = instruction_idx % instructions_len;

        let curr_instruction = instructions[instruction_idx];

        let node_elems = &node_map[curr_node];

        curr_node = match curr_instruction {
            'L' => &node_elems.0,
            'R' => &node_elems.1,
            _ => unreachable!(),
        };

        instruction_idx += 1;
        n_steps += 1;
    }

    n_steps
}

fn part2(instructions: &Vec<char>, node_map: &NodeMap) -> u64 {
    let instructions_len = instructions.len();
    let mut curr_nodes: Vec<&str> = node_map
        .keys()
        .filter(|&n| n.ends_with("A"))
        .map(|n| n.as_str())
        .collect();

    let mut path_lengths: Vec<u64> = Vec::new();

    for node in curr_nodes.iter_mut() {
        let mut instruction_idx = 0;
        let mut n_steps = 0;
        let mut reached_end = false;

        while !reached_end {
            let curr_instruction = instructions[instruction_idx];
            let node_elems = &node_map[*node];
            *node = match curr_instruction {
                'L' => &node_elems.0,
                'R' => &node_elems.1,
                _ => unreachable!(),
            };

            if node.ends_with("Z") {
                reached_end = true;
            }

            n_steps += 1;
            instruction_idx += 1;
            instruction_idx = instruction_idx % instructions_len;
        }

        path_lengths.push(n_steps);
    }

    lcm(&path_lengths)
}

pub fn main() {
    let input = String::from_utf8(include_bytes!("sample_input_data/day8.txt").to_vec()).unwrap();
    let (instructions, node_map) = parse_input(&input);

    //println!("{}", part1(&instructions, &node_map));
    println!("{}", part2(&instructions, &node_map));
}
