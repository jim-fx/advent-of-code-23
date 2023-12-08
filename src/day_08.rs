use std::collections::HashMap;

pub fn solve(input: String) -> (u32, u32) {
    let mut lines = input.lines().collect::<Vec<&str>>();

    let directions = lines.remove(0).chars().collect::<Vec<char>>();
    let direction_count = directions.len() as u32;

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut pos = "AAA";

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

        map.insert(&key, (l, r));
    }

    let mut index: u32 = 0;

    let mut steps = 0;

    while pos != "ZZZ" {
        let dir = directions.get(index as usize).unwrap();

        pos = match dir {
            'L' => map.get(pos).unwrap().0,
            'R' => map.get(pos).unwrap().1,
            _ => panic!("Unknown direction"),
        };

        index = (index + 1) % direction_count;
        steps += 1;
    }

    return (steps, 0);
}
