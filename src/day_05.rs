fn convert_seeds(seeds: Vec<u64>, converters: Vec<[u64; 3]>) -> Vec<u64> {
    return seeds
        .iter()
        .map(|seed| {
            for converter in &converters {
                if seed >= &converter[0] && seed <= &converter[1] {
                    let offset = seed - converter[0];
                    let out = converter[2] + offset;
                    return out;
                }
            }
            return *seed;
        })
        .collect::<Vec<u64>>();
}

pub fn solve(input: String) -> (u32, u32) {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut seeds = lines[0]
        .replace("seeds: ", "")
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut converters: Vec<[u64; 3]> = Vec::new();

    for line_index in 1..lines.len() {
        let line = lines[line_index];

        if line.len() == 0 {
            continue;
        }

        if line.chars().last().unwrap() == ':' {
            if converters.len() > 0 {
                seeds = convert_seeds(seeds, converters.clone());
                converters = Vec::new();
            }
            continue;
        }
        let line_group = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let in_start = line_group[1];
        let in_end = in_start + line_group[2];
        let out_end = line_group[0];

        let out = [in_start, in_end, out_end];

        converters.push(out);
    }

    // Convert one final time, because the last group has not been appplied
    seeds = convert_seeds(seeds, converters.clone());

    let min_seed = seeds.iter().min().unwrap();

    println!("{:?}", min_seed);

    return (0, 0);
}
