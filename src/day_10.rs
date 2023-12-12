use std::cmp;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    NONE = 0,
    NORTH = 1,
    EAST = 2,
    SOUTH = 3,
    WEST = 4,
}

fn parse_input(input: String) -> (Vec<Vec<usize>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .map(|(char_index, c)| match c {
                    '┐' => 6,
                    '┌' => 5,
                    '┘' => 4,
                    '└' => 3,
                    '-' => 2,
                    '|' => 1,
                    'S' => {
                        start = (line_index, char_index);
                        return 1;
                    }
                    _ => 0,
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let width = map[0].len();
    let height = map.len();

    return (map, start, (width, height));
}

pub fn solve(input: String) -> (u64, u64) {
    // println!("{}", input);

    let tile_types = [
        (Direction::NONE, Direction::NONE),   // 0: .
        (Direction::NORTH, Direction::SOUTH), // 1: |
        (Direction::EAST, Direction::WEST),   // 2: -
        (Direction::NORTH, Direction::EAST),  // 3: └
        (Direction::NORTH, Direction::WEST),  // 4: ┘
        (Direction::EAST, Direction::SOUTH),  // 5: ┌
        (Direction::SOUTH, Direction::WEST),  // 6: ┐
    ];

    let (map, start, dims) = parse_input(input);

    // for line in &map {
    //     println!("{:?}", line);
    // }

    let mut pos = start.clone();
    let mut dir = &Direction::NONE;
    let mut depth = 0;

    // find first direction
    for i in 0..8 {
        let current_tile = map[pos.0 as usize][pos.1 as usize];
        let new_pos = match i {
            0 => {
                // NORTH
                if pos.0 == 0 {
                    continue;
                }
                (pos.0 - 1 as usize, pos.1 as usize)
            }
            1 => {
                //EAST
                if pos.1 + 1 >= dims.1 {
                    continue;
                }
                (pos.0 as usize, pos.1 + 1 as usize)
            }
            2 => {
                // SOUTH
                if pos.0 + 1 >= dims.0 {
                    continue;
                }
                (pos.0 + 1 as usize, pos.1 as usize)
            }
            3 => {
                // WEST}
                if pos.1 == 0 {
                    continue;
                }
                (pos.0 as usize, pos.1 - 1 as usize)
            }
            _ => panic!("Unknown tile: {}", current_tile),
        };

        let next_tile = map[new_pos.0][new_pos.1];

        if next_tile != 0 {
            dir = match next_tile {
                1 => &Direction::NORTH,
                2 => &Direction::EAST,
                3 => &Direction::SOUTH,
                4 => &Direction::WEST,
                _ => panic!("Unknown tile: {}", current_tile),
            };
            break;
        }
    }

    // println!("start_pos:{:?}, dir:{:?}", pos, dir);

    // this is the bounding box of the path
    let mut min = (dims.1, dims.0);
    let mut max = (0, 0);

    let mut path: HashMap<usize, HashSet<usize>> = HashMap::new();

    while dir != &Direction::NONE {
        min.0 = cmp::min(min.0, pos.0);
        min.1 = cmp::min(min.1, pos.1);
        max.0 = cmp::max(max.0, pos.0);
        max.1 = cmp::max(max.1, pos.1);
        let entry = path.entry(pos.0).or_insert(HashSet::new());
        entry.insert(pos.1);

        let next_pos = match dir {
            Direction::NORTH => (pos.0 - 1 as usize, pos.1 as usize),
            Direction::EAST => (pos.0 as usize, pos.1 + 1 as usize),
            Direction::SOUTH => (pos.0 + 1 as usize, pos.1 as usize),
            Direction::WEST => (pos.0 as usize, pos.1 - 1 as usize),
            _ => panic!("Unknown direction: {:?}", dir),
        };

        let invert_dir = match dir {
            Direction::NORTH => &Direction::SOUTH,
            Direction::EAST => &Direction::WEST,
            Direction::SOUTH => &Direction::NORTH,
            Direction::WEST => &Direction::EAST,
            _ => panic!("Unknown direction: {:?}", dir),
        };

        let tile = &tile_types[map[next_pos.0][next_pos.1]];
        let next_dir = if tile.0 == *invert_dir {
            &tile.1
        } else {
            &tile.0
        };

        // println!(
        //     "next_pos:{:?}, tile:{:?}, next_dir:{:?}",
        //     next_pos, tile, next_dir
        // );

        pos = next_pos;
        dir = next_dir;

        depth += 1;
    }

    let bounding_box = (max.0 - min.0 + 1, max.1 - min.1 + 1);
    let amount = bounding_box.0 * bounding_box.1;

    let mut line_space_index = 0;
    let mut on_path = false;
    let mut inside_count = 0;
    let mut inside = false;
    let mut is_rising = false;

    for i in 0..amount {
        let x = min.1 + i % bounding_box.1;
        let y = min.0 + i / bounding_box.1;

        let tile = tile_types[map[y][x]];

        if x == min.1 {
            on_path = false;
            line_space_index = 0;
            print!("\n");
        }

        let mut _on_path = false;
        if let Some(set) = path.get(&y) {
            _on_path = set.contains(&x);
        }

        if on_path && !_on_path {
            // line_space_index += 1;
            // print!("\\");
            // print!("-");
        } else if !on_path && _on_path {
            // print!("/");
        } else {
            if line_space_index % 2 != 0 {
                // print!("-");
            } else {
                // print!("x");
            }
        }

        if tile.0 == Direction::NORTH
            || tile.1 == Direction::NORTH
            || tile.0 == Direction::SOUTH
            || tile.1 == Direction::SOUTH
        {
            if _on_path != on_path {
                line_space_index += 1;
            }
        }
        if !_on_path {
            print!("{}", line_space_index);
        } else {
            print!(".");
        }

        on_path = _on_path;

        if on_path {
            // print!("█");
        }
        if inside && !on_path && line_space_index % 2 != 0 {
            inside_count += 1;
            // print!("X");
        } else {
            if !on_path {
                // print!(" ");
            }
        }
    }
    println!("");

    // println!("depth: {}", depth);

    return (depth / 2 as u64, inside_count as u64 + 1);
}
