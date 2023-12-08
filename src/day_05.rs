use std::u64::MAX;

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

pub fn solve(input: String) -> (u64, u64) {
    let lines = input.lines().collect::<Vec<&str>>();

    let input_seeds = lines[0]
        .replace("seeds: ", "")
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut seeds = input_seeds.clone();

    let mut all_converters: Vec<Vec<[u64; 3]>> = Vec::new();
    let mut converters: Vec<[u64; 3]> = Vec::new();

    for line_index in 1..lines.len() {
        let line = lines[line_index];

        if line.len() == 0 {
            continue;
        }

        if line.chars().last().unwrap() == ':' {
            if converters.len() > 0 {
                seeds = convert_seeds(seeds, converters.clone());
                all_converters.push(converters.clone());
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

    all_converters.push(converters.clone());
    // Convert one final time, because the last group has not been appplied
    seeds = convert_seeds(seeds, converters.clone());
    let min_seed = seeds.iter().min().unwrap();
    println!("min seed: {}", min_seed);

    let mut min_new_seed = MAX;

    let mut amount: u128 = 0;

    let mut amount_converters = 0;

    for group in &all_converters {
        amount_converters += group.len() * 3;
    }

    println!("amount converters: {}", amount_converters);

    for seed_index in 0..input_seeds.len() {
        if seed_index % 2 != 0 {
            println!("amount: {}", amount);
            let start = input_seeds[seed_index - 1];
            let range = input_seeds[seed_index];

            println!("range: {}", range);

            for _seed in start..start + range {
                amount += 1;
                let mut seed = _seed;
                for converters in &all_converters {
                    for converter in converters {
                        if &seed >= &converter[0] && &seed <= &converter[1] {
                            let offset = seed - converter[0];
                            let out = converter[2] + offset;
                            seed = out;
                            break;
                        }
                    }
                }
                if seed < min_new_seed {
                    min_new_seed = seed;
                }
            }
        }
    }

    println!("min new seed: {}", min_new_seed);

    return (0, 0);
}
