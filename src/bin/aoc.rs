use aoc2020::day1;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    if let Some(day) = env::args().nth(1) {
        let path = format!("input/{}", day);
        let file = File::open(path).expect("File not found");
        let reader = BufReader::new(file);

        let input = reader.lines().map(|line| line.unwrap()).collect();

        println!("- {}:", day);
        match day.as_ref() {
            "day1" => day1::run(input),
            _ => println!("{}, did not match :(", day),
        }
    }
}
