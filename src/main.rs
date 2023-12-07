use std::fs::File;
use std::io::Read;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let first_arg = format!("{:0>2}", args.remove(1));
    let day_arg = first_arg.as_str();
    let test_arg = args.iter().find(|&arg| arg == "--test");

    let suffix = if test_arg.is_some() { "_test" } else { "" };

    let input_file = format!("./inputs/{day_arg}{suffix}.txt");

    println!("{} {}", day_arg, input_file);

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
        _ => day_01::solve(input_buffer),
    };

    let end_time = std::time::Instant::now();

    println!("Time elapsed: {:?}", end_time - start_time);
    println!("Part 1: {}", result_01);
    println!("Part 2: {}", result_02);
}
