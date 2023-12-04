use regex::Regex;

pub fn solve(input: String) {
    let lines = input.split("\n");

    //12 red cubes, 13 green cubes, and 14 blue
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let game_regex = Regex::new(r"(^Game \d+: |,)").expect("Invalid regex pattern");
    let mut index = 1;
    let mut total = 0;
    let mut power = 0;

    for line in lines.into_iter() {
        if line.len() == 0 {
            continue;
        }

        let clean_line = game_regex.replace_all(line, "").replace(";", " ;");

        let mut valid = true;
        let mut num = 0;
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let mut min_red = 1;
        let mut min_green = 1;
        let mut min_blue = 1;

        for word in clean_line.split(" ") {
            if !valid {
                break;
            }
            match word {
                "red" => {
                    red = num;
                    if num > min_red {
                        min_red = num;
                    }
                    num = 0;
                }
                "green" => {
                    green = num;
                    if num > min_green {
                        min_green = num;
                    }
                    num = 0;
                }
                "blue" => {
                    blue = num;
                    if num > min_blue {
                        min_blue = num;
                    }
                    num = 0;
                }
                ";" => {
                    blue = 0;
                    red = 0;
                    green = 0;
                    if red > max_red || green > max_green || blue > max_blue {
                        valid = false;
                    }
                }
                _ => {
                    num = word.parse::<i32>().expect("Failed to parse");
                }
            }
        }

        if valid {
            total += index;
        }

        power += min_red * min_green * min_blue;

        index += 1;
    }

    println!("Total: {}", total);
    println!("Power: {}", power);
}
