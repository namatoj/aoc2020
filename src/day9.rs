use itertools::Itertools;
use std::collections::VecDeque;

pub fn run(input: Vec<String>) {
    println!("Day 9");
    println!("=====");

    println!("part one: {}", part_one(&input, 25));
    println!("part two: {}", part_two(&input, 25));
}

fn find_fist_non_valid(input: Vec<u64>, preamble: usize) -> u64 {
    let mut it = input.iter();
    let mut numbers: VecDeque<u64> = VecDeque::new();

    // reading preamble
    for _ in 0..preamble {
        numbers.push_back(*it.next().unwrap());
    }

    for x in it {
        let combinations = numbers.iter().combinations(2);

        let sums: Vec<u64> = combinations.map(|x| x.iter().copied().sum()).collect();

        if !sums.contains(x) {
            return *x;
        }

        numbers.push_back(*x);
        numbers.pop_front();
    }

    panic!("all elements were valid");
}

pub fn part_one(input: &[String], preamble: usize) -> u64 {
    let input: Vec<_> = input.iter().map(|x| x.parse::<u64>().unwrap()).collect();

    find_fist_non_valid(input, preamble)
}

pub fn part_two(input: &[String], preamble: usize) -> u64 {
    let input: Vec<_> = input.iter().map(|x| x.parse::<u64>().unwrap()).collect();
    let goal = find_fist_non_valid(input.clone(), preamble);

    println!("goal: {}", goal);

    'outer: for window_length in 2..input.len() {
        for window in input.windows(window_length) {
            if window.contains(&goal) {
                continue 'outer;
            }

            let sum: u64 = window.iter().sum();

            if sum == goal {
                return window.iter().max().unwrap() + window.iter().min().unwrap();
            }
        }
    }
    panic!("couldn't find the weakness");
}

#[test]
fn part1_example_1() {
    let v = vec![
        "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
        "127", "219", "299", "277", "309", "576",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(127, part_one(&v, 5));
}

#[test]
fn part2_example_1() {
    let v = vec![
        "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
        "127", "219", "299", "277", "309", "576",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(62, part_two(&v, 5));
}
