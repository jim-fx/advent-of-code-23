use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref GAME_PATTERN: regex::Regex =
        Regex::new(r"^Card +\d+: ").expect("Invalid regex pattern");
}

fn calculate_value(amount: u32) -> u32 {
    if amount == 0 {
        return 0;
    }
    if amount == 1 {
        return 1;
    }
    let base: u32 = 2;
    return base.pow(amount - 1);
}

fn get_amount_winning(line: &str) -> usize {
    let s = line.split('|');

    let winning_numbers = format!(" {}", s.clone().nth(0).unwrap());
    let numbers = s.clone().nth(1).unwrap().split(" ");

    let mut total = 0;

    for number in numbers {
        if number.len() == 0 || number == " " {
            continue;
        }
        let n = format!(" {} ", number);
        if winning_numbers.contains(&n) {
            total += 1;
        }
    }

    return total;
}

pub fn solve(input: String) -> (u32, u32) {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut total = 0;

    let mut copies_per_line: Vec<u32> = lines.iter().map(|_| 1).collect();

    for line_index in 0..lines.len() {
        let _line = lines[line_index];

        let line = GAME_PATTERN.replace_all(_line, "").to_string();

        if line.len() == 0 {
            continue;
        }

        // Get amount of winning numbers
        let amount = get_amount_winning(line.as_str());

        // println!(
        //     "index: {} amount: {}, copies_per_line: {}",
        //     line_index, amount, copies_per_line[line_index]
        // );

        let _value = calculate_value(amount as u32);

        // println!("value: {}", value);

        for index_offset in 0..amount {
            let offset_index = line_index + index_offset + 1;

            copies_per_line[offset_index] += copies_per_line[line_index]
        }
    }

    for num in copies_per_line {
        total += num;
    }
    return (0, total - 1);
}
