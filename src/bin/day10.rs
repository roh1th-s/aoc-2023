#[derive(Debug, Clone)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NEBend,
    NWBend,
    SEBend,
    SWBend,
    Ground,
    StartPos,
}

use Tile::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord2D {
    x: u64,
    y: u64,
}

fn parse_grid(input: &String) -> (Vec<Vec<Tile>>, Coord2D) {
    let mut start_coord: Coord2D = Coord2D { x: 0, y: 0 };

    let tile_grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, chr)| match chr {
                    '|' => VerticalPipe,
                    '-' => HorizontalPipe,
                    'L' => NEBend,
                    'J' => NWBend,
                    '7' => SWBend,
                    'F' => SEBend,
                    '.' => Ground,
                    'S' => {
                        start_coord = Coord2D {
                            x: x as u64,
                            y: y as u64,
                        };
                        StartPos
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<Tile>>()
        })
        .collect();

    (tile_grid, start_coord)
}

fn find_connecting_pipe(
    tile_grid: &Vec<Vec<Tile>>,
    tile_coord: Coord2D,
    last_visited: Coord2D,
) -> Option<(&Tile, Coord2D)> {
    // okay, this is way more verbose than it needs to be...but wtever

    let tile_x = tile_coord.x as usize;
    let tile_y = tile_coord.y as usize;
    let tile = &tile_grid[tile_y][tile_x];
    let last_visited_x = last_visited.x as usize;
    let last_visited_y = last_visited.y as usize;

    // look up
    if let VerticalPipe | NEBend | NWBend | StartPos = tile {
        if tile_y > 0 && !(last_visited_y == tile_y - 1 && last_visited_x == tile_x) {
            let tile_above = &tile_grid[tile_y - 1][tile_x];
            if let VerticalPipe | SEBend | SWBend | StartPos = tile_above {
                return Some((
                    tile_above,
                    Coord2D {
                        x: tile_x as u64,
                        y: (tile_y - 1) as u64,
                    },
                ));
            }
        }
    }

    // look down
    if let VerticalPipe | SWBend | SEBend | StartPos = tile {
        if tile_y < tile_grid.len() - 1
            && !(last_visited_y == tile_y + 1 && last_visited_x == tile_x)
        {
            let tile_below = &tile_grid[tile_y + 1][tile_x];

            if let VerticalPipe | NEBend | NWBend | StartPos = tile_below {
                return Some((
                    tile_below,
                    Coord2D {
                        x: tile_x as u64,
                        y: (tile_y + 1) as u64,
                    },
                ));
            }
        }
    }

    // look left
    if let HorizontalPipe | NWBend | SWBend | StartPos = tile {
        if tile_x > 0 && !(last_visited_y == tile_y && last_visited_x == tile_x - 1) {
            let tile_left = &tile_grid[tile_y][tile_x - 1];

            if let HorizontalPipe | SEBend | NEBend | StartPos = tile_left {
                return Some((
                    tile_left,
                    Coord2D {
                        x: (tile_x - 1) as u64,
                        y: tile_y as u64,
                    },
                ));
            }
        }
    }

    // look right
    if let HorizontalPipe | NEBend | SEBend | StartPos = tile {
        if tile_x < tile_grid[0].len() - 1
            && !(last_visited_y == tile_y && last_visited_x == tile_x + 1)
        {
            let tile_right = &tile_grid[tile_y][tile_x + 1];

            if let HorizontalPipe | SWBend | NWBend | StartPos = tile_right {
                return Some((
                    tile_right,
                    Coord2D {
                        x: (tile_x + 1) as u64,
                        y: tile_y as u64,
                    },
                ));
            }
        }
    }
    None
}

fn get_segment_in_between(before_coord: Coord2D, after_coord: Coord2D) -> Tile {
    if after_coord.x == before_coord.x {
        VerticalPipe
    } else if after_coord.y == before_coord.y {
        HorizontalPipe
    } else if after_coord.x < before_coord.x {
        if after_coord.y < before_coord.y {
            NWBend
        } else {
            SWBend
        }
    } else {
        if after_coord.y < before_coord.y {
            NEBend
        } else {
            SEBend
        }
    }
}
fn get_pipe(tile_grid: &Vec<Vec<Tile>>, start_coord: Coord2D) -> Vec<Coord2D> {
    let mut pipe_coords = vec![start_coord];
    let mut last_visited = start_coord;

    // get first connecting pipe
    let (_, mut curr_coord) = find_connecting_pipe(tile_grid, start_coord, last_visited).unwrap();

    // keep moving through the pipes until we return to start
    while !matches!(
        tile_grid[curr_coord.y as usize][curr_coord.x as usize],
        StartPos
    ) {
        let (_, next_coord) = find_connecting_pipe(tile_grid, curr_coord, last_visited).unwrap();
        pipe_coords.push(curr_coord);
        last_visited = curr_coord;
        curr_coord = next_coord;
    }

    pipe_coords
}

fn part1(tile_grid: &Vec<Vec<Tile>>, start_coord: Coord2D) -> u64 {
    // divide number of pipe segments by 2
    get_pipe(tile_grid, start_coord).len() as u64 / 2
}

fn part2(tile_grid: &Vec<Vec<Tile>>, start_coord: Coord2D) -> u64 {
    // check if number of pipe passes on one side of a point is odd
    // this will tell us if the point is enclosed by the pipe
    // (the following implementation is not exactly efficient but eh)
    
    let tile_grid_updated = &mut (*tile_grid).clone();
    let pipe = get_pipe(tile_grid, start_coord);

    // replace all non pipe tiles with ground
    for y in 0..tile_grid.len() {
        for x in 0..tile_grid[0].len() {
            if !pipe.contains(&Coord2D {
                x: x as u64,
                y: y as u64,
            }) {
                tile_grid_updated[y][x] = Ground;
            }
        }
    }

    // replace start position with the right pipe segment
    tile_grid_updated[start_coord.y as usize][start_coord.x as usize] =
        get_segment_in_between(pipe[1], pipe[pipe.len() - 2]);

    let mut n_enclosed_points = 0;

    for y in 0..tile_grid_updated.len() {
        for x in 0..tile_grid_updated[0].len() {
            if matches!(tile_grid_updated[y][x], Ground) {
                // check all points not including the pipe

                let mut no_of_passes = 0;
                let mut last_pipe_bend: Option<&Tile> = None;

                for i in x + 1..tile_grid_updated[0].len() {
                    match tile_grid_updated[y][i] {
                        VerticalPipe => no_of_passes += 1,
                        NEBend | SEBend => last_pipe_bend = Some(&tile_grid_updated[y][i]),
                        NWBend => {
                            if let Some(pipe_bend) = last_pipe_bend {
                                if matches!(pipe_bend, SEBend) {
                                    no_of_passes += 1;
                                    last_pipe_bend = None;
                                }
                            }
                        }
                        SWBend => {
                            if let Some(pipe_bend) = last_pipe_bend {
                                if matches!(pipe_bend, NEBend) {
                                    no_of_passes += 1;
                                    last_pipe_bend = None;
                                }
                            }
                        }
                        _ => continue,
                    }
                }

                if no_of_passes % 2 != 0 {
                    n_enclosed_points += 1;
                }
            }
        }
    }

    n_enclosed_points
}
pub fn main() {
    let input = String::from_utf8(include_bytes!("sample_input_data/day10.txt").to_vec()).unwrap();
    let (tile_grid, start_coord) = parse_grid(&input);
    println!("{}", part1(&tile_grid, start_coord));
    println!("{}", part2(&tile_grid, start_coord));
}
