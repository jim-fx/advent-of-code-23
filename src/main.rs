use std::io::{self, Read};

pub mod a;
pub mod b;

fn main() {
    // Read from stdin
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");

    // Extract the command line argument after "--day="
    let args: Vec<String> = std::env::args().collect();
    let day_arg = args.iter().find(|&arg| arg.starts_with("--day="));

    if let Some(day_arg) = day_arg {
        let day_value: String = day_arg.chars().skip(6).collect();
        if day_value == "a" {
            a::solve(buffer);
        } else if day_value == "b" {
            b::solve(buffer)
        } else {
            eprintln!("Error: --day argument must be 'a' or 'b'");
        }
    } else {
        eprintln!("Error: --day argument not found");
    }
}
