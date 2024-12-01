use std::env;
use std::fs;

mod aoc01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let challenge = &args[1];
    let filename = format!("input/input_{}.txt", challenge);
    let input = fs::read_to_string(filename);
    let data = input.unwrap();
    let answer = match challenge.as_str() {
        "01" => aoc01::aoc(data),
        _ => (0, 0),
    };
    println!("{:?}", answer)
}
