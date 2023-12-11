use std::fs::File;
use std::io::Read;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let first_arg = format!("{:0>2}", args.remove(1));
    let day_arg = first_arg.as_str();
    let test_arg = args.iter().find(|&arg| arg == "--test");

    let suffix = if test_arg.is_some() { "_test" } else { "" };

    let input_file = format!("./inputs/{day_arg}{suffix}.txt");

    let mut input_buffer = String::new();
    File::open(input_file)
        .unwrap()
        .read_to_string(&mut input_buffer)
        .unwrap();

    let start_time = std::time::Instant::now();

    let (result_01, result_02) = match day_arg {
        "01" => day_01::solve(input_buffer),
        "02" => day_02::solve(input_buffer),
        "03" => day_03::solve(input_buffer),
        "04" => day_04::solve(input_buffer),
        "05" => day_05::solve(input_buffer),
        "06" => day_06::solve(input_buffer),
        "07" => day_07::solve(input_buffer),
        "08" => day_08::solve(input_buffer),
        "09" => day_09::solve(input_buffer),
        "10" => day_10::solve(input_buffer),
        "11" => day_11::solve(input_buffer),
        "12" => day_12::solve(input_buffer),
        "13" => day_13::solve(input_buffer),
        "14" => day_14::solve(input_buffer),
        "15" => day_15::solve(input_buffer),
        "16" => day_16::solve(input_buffer),
        "17" => day_17::solve(input_buffer),
        "18" => day_18::solve(input_buffer),
        "19" => day_19::solve(input_buffer),
        "20" => day_20::solve(input_buffer),
        "21" => day_21::solve(input_buffer),
        "22" => day_22::solve(input_buffer),
        "23" => day_23::solve(input_buffer),
        "24" => day_24::solve(input_buffer),
        _ => day_01::solve(input_buffer),
    };

    let end_time = std::time::Instant::now();

    println!("Time elapsed: {:?}", end_time - start_time);
    println!("Part 1: {}", result_01);
    println!("Part 2: {}", result_02);
}
