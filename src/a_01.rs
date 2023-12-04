use fancy_regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn parse_num(num: &str) -> u32 {
    match num {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}
pub fn solve() -> io::Result<u32> {
    // Your regex pattern
    let regex_pattern = r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))";
    let regex = Regex::new(regex_pattern).expect("Invalid regex pattern");

    // Path to your text file
    let path = Path::new("./src/a_input.txt");
    let file = File::open(&path)?;

    let reader = BufReader::new(file);

    let mut total = 0;

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;
        let mut first_num: u32 = 0;
        let mut last_num: u32 = 0;
        // Iterate over matches in the line
        for mat in regex.captures_iter(&line) {
            let m = mat.expect("err").get(1).unwrap().as_str();
            let num = parse_num(m);
            if num != 0 {
                if first_num == 0 {
                    first_num = num;
                    last_num = num;
                } else {
                    last_num = num;
                }
            }
        }
        println!("{}", line);
        println!("{} {}", first_num, last_num);
        total += first_num * 10 + last_num;
    }

    Ok(total)
}
