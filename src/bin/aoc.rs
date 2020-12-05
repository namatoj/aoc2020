use aoc2020::day1;
use aoc2020::day2;
use aoc2020::day3;
use aoc2020::day4;

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

        match day.as_ref() {
            "day1" => day1::run(input),
            "day2" => day2::run(input),
            "day3" => day3::run(input),
            "day4" => day4::run(input),
            _ => println!("{}, did not match :(, did you add it do aoc.rs", day),
        }
    }
}
