use regex::Regex;

pub fn solve(input: String) -> (u64, u64) {
    let lines = input.split("\n");

    //12 red cubes, 13 green cubes, and 14 blue
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let game_regex = Regex::new(r"(^Game \d+: |,)").expect("Invalid regex pattern");
    let mut index = 1;
    let mut total: u64 = 0;
    let mut power: u64 = 0;

    for line in lines.into_iter() {
        if line.len() == 0 {
            continue;
        }

        let clean_line = game_regex.replace_all(line, "").replace(";", " ;");

        let mut valid = true;
        let mut num = 0;
        let mut _red = 0;
        let mut _green = 0;
        let mut _blue = 0;

        let mut min_red = 1;
        let mut min_green = 1;
        let mut min_blue = 1;

        for word in clean_line.split(" ") {
            if !valid {
                break;
            }
            match word {
                "red" => {
                    _red = num;
                    if num > min_red {
                        min_red = num;
                    }
                    num = 0;
                }
                "green" => {
                    _green = num;
                    if num > min_green {
                        min_green = num;
                    }
                    num = 0;
                }
                "blue" => {
                    _blue = num;
                    if num > min_blue {
                        min_blue = num;
                    }
                    num = 0;
                }
                ";" => {
                    _blue = 0;
                    _red = 0;
                    _green = 0;
                    if _red > max_red || _green > max_green || _blue > max_blue {
                        valid = false;
                    }
                }
                _ => {
                    num = word.parse::<u64>().expect("Failed to parse");
                }
            }
        }

        if valid {
            total += index;
        }

        power += min_red * min_green * min_blue;

        index += 1;
    }

    return (total, power);
}
