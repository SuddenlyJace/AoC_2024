use std::env;
use std::fs;

mod aoc01;
//mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;

fn main() {
    let args: Vec<String> = env::args().collect();
    let challenge = &args[1];
    let filename = format!("input/input_{}.txt", challenge);
    let input = fs::read_to_string(filename);
    let data = input.unwrap();
    let answer = match challenge.as_str() {
        "01" => aoc01::aoc(data),
        //"02" => aoc02::aoc(data),
        "03" => aoc03::aoc(data),
        "04" => aoc04::aoc(data),
        "05" => aoc05::aoc(data),
        _ => (0, 0),
    };
    println!("{:?}", answer)
}
