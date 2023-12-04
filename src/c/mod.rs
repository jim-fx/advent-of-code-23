use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};

lazy_static! {
    static ref SYMBOL_PATTERN: regex::Regex =
        Regex::new(r"[^0-9.]").expect("Invalid regex pattern");
}

pub fn solve(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();

    let line_count = lines.len();

    let mut total = 0;
    let mut gear_ration = 0;

    let mut line_index = 0;
    for line in lines.clone().into_iter() {
        if line.len() == 0 {
            continue;
        }

        // here we will store the numbers with the start and end indices
        let mut numbers: Vec<(usize, usize, i32)> = vec![];

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
                numbers.push((start, end, sum.parse::<i32>().unwrap()));
                sum.clear();
                start = 0;
                end = 0;
            }
            char_index += 1;
        }
        if start != 0 {
            numbers.push((start, end, sum.parse::<i32>().unwrap()));
        }

        // check if there are any symbols around the numbers
        for num in numbers {
            let start_index = num.0;
            let end_index = num.1;
            let sum = num.2;

            let start_line_index = max(line_index, 1) - 1;
            let end_line_index = min(line_index + 2, line_count - 1);

            let sub_lines = lines[start_line_index..end_line_index].to_vec();

            for line in sub_lines {
                let part = &line[start_index..end_index];
                if SYMBOL_PATTERN.is_match(part) {
                    total += sum;
                    break;
                }

                // two part number
                if sum < 100 && sum > 9 {}
            }
        }

        line_index += 1;
    }

    println!("{}", total);
}
