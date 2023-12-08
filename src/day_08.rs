use num::integer::lcm;
use std::collections::HashMap;

fn lcm_of_vec(numbers: Vec<u64>) -> u64 {
    if numbers.is_empty() {
        panic!("Cannot calculate LCM for an empty vector");
    }

    // Calculate LCM using the fold function and the lcm method from the num crate
    numbers.into_iter().fold(1, |acc, x| lcm(acc, x))
}
pub fn solve(input: String) -> (u64, u64) {
    let mut lines = input.lines().collect::<Vec<&str>>();

    let directions = lines.remove(0).chars().collect::<Vec<char>>();
    let direction_count = directions.len() as u64;

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut positions: Vec<&str> = Vec::new();

    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }
        let result: Vec<&str> = line
            .split(|c| c == '(' || c == ')' || c == ',' || c == '=')
            .map(str::trim)
            .collect();

        let key = result.get(0).unwrap();
        let l = result.get(2).unwrap();
        let r = result.get(3).unwrap();

        if key.ends_with('A') {
            positions.push(key);
        }

        map.insert(&key, (l, r));
    }

    let mut index: u64 = 0;

    let mut steps = 0;
    let mut multiples: Vec<u64> = Vec::new();

    while positions.len() > 0 {
        let dir = directions.get(index as usize).unwrap();

        positions = positions
            .into_iter()
            .map(|p| {
                if p.is_empty() {
                    return "000";
                }
                let line = map.get(p).unwrap();
                let new_pos = match dir {
                    'L' => line.0,
                    'R' => line.1,
                    _ => panic!("Unknown direction"),
                };

                if new_pos.ends_with('Z') {
                    multiples.push(steps + 1);
                    return "000";
                }

                return new_pos;
            })
            .filter(|p| p != &"000")
            .collect();

        index = (index + 1) % direction_count;
        steps += 1;
    }
    let result = lcm_of_vec(multiples);

    return (steps as u64, result as u64);
}
