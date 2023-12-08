use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;

lazy_static! {
    static ref SYMBOL_PATTERN: regex::Regex =
        Regex::new(r"[^0-9.]").expect("Invalid regex pattern");
}

pub fn solve(input: String) -> (u64, u64) {
    let lines: Vec<&str> = input.split("\n").collect();

    let line_count = lines.len();

    let mut total = 0;
    let mut total_gear = 0;

    // line_index|char_index -> vector<partnumbers>
    let mut gears: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    let mut line_index = 0;
    for line in lines.clone().into_iter() {
        if line.len() == 0 {
            continue;
        }

        // here we will store the numbers with the start and end indices
        let mut numbers: Vec<(usize, usize, u64)> = vec![];

        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut sum = String::new();

        let mut char_index = 0;
        for char in line.chars() {
            // if the char is a digit
            if char.is_digit(10) {
                if end == 0 {
                    start = max(char_index, 1) - 1;
                    end = min(char_index + 2, line.len())
                } else {
                    end = min(char_index + 2, line.len())
                }
                sum.push(char);
            // if we reach the end of a number
            } else if end != 0 {
                numbers.push((start, end, sum.parse::<u64>().unwrap()));
                sum.clear();
                start = 0;
                end = 0;
            }
            char_index += 1;
        }
        if start != 0 {
            numbers.push((start, end, sum.parse::<u64>().unwrap()));
        }

        // check if there are any symbols around the numbers
        for num in numbers {
            let start_index = num.0;
            let end_index = num.1;
            let sum = num.2;

            let start_line_index = max(line_index, 1) - 1;
            let end_line_index = min(line_index + 2, line_count - 1);

            let sub_lines = lines[start_line_index..end_line_index].to_vec();

            let mut found = false;
            let mut subline_index = max(line_index, 1) - 1;
            for line in sub_lines {
                let part = &line[start_index..end_index];
                if SYMBOL_PATTERN.is_match(part) && !found {
                    total += sum;
                    found = true;
                }
                if let Some(char_index) = part.find("*") {
                    let gear_id = (subline_index - 1, start_index + char_index);
                    // Check if the key exists in the HashMap
                    if let Some(vec) = gears.get_mut(&gear_id) {
                        // If it exists, push the value to the Vec
                        vec.push(sum);
                    } else {
                        // If it doesn't exist, create a new entry with a Vec containing the single int
                        let new_vec = vec![sum];
                        gears.insert(gear_id, new_vec);
                    }
                }
                subline_index += 1;
            }
        }

        line_index += 1;
    }

    let filtered_values: HashMap<_, _> = gears
        .into_iter()
        .filter(|(_, vec)| vec.len() == 2)
        .collect();

    let gear_numbers: Vec<Vec<u64>> = filtered_values.values().cloned().collect();

    for gear in gear_numbers {
        total_gear += gear[0] * gear[1];
    }

    return (total, total_gear);
}
