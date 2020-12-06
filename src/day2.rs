use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum ParsePasswordError {
    ParseError,
}

#[derive(Debug)]
struct Password {
    lower_limit: usize,
    upper_limit: usize,
    letter: char,
    password: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let x = self.password.matches(self.letter).count();

        (self.lower_limit <= x) && (x <= self.upper_limit)
    }

    fn is_valid_part_two(&self) -> bool {
        let chars: Vec<_> = self.password.chars().collect();

        (chars[self.lower_limit - 1] == self.letter) ^ (chars[self.upper_limit - 1] == self.letter)
    }
}

impl From<ParseIntError> for ParsePasswordError {
    fn from(_err: ParseIntError) -> Self {
        ParsePasswordError::ParseError
    }
}

impl FromStr for Password {
    type Err = ParsePasswordError;

    fn from_str(s: &str) -> Result<Password, Self::Err> {
        let mut it = s.split_whitespace();
        let limits = it.next().unwrap();
        let letter = it
            .next()
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse::<char>()
            .unwrap();
        let password = it.next().unwrap().to_string();

        let mut limit_iterator = limits.split('-');
        let lower_limit = limit_iterator.next().unwrap();
        let upper_limit = limit_iterator.next().unwrap();

        let lower_limit = lower_limit.parse::<usize>()?;
        let upper_limit = upper_limit.parse::<usize>()?;

        Ok(Self {
            lower_limit,
            upper_limit,
            letter,
            password,
        })
    }
}

pub fn run(input: Vec<String>) {
    println!("Day 2");
    println!("=====");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

pub fn part_one(input: &[String]) -> usize {
    let passwords: Vec<Password> = input
        .iter()
        .map(|x| x.parse::<Password>().unwrap())
        .collect();

    passwords.iter().filter(|pass| pass.is_valid()).count()
}

pub fn part_two(input: &[String]) -> usize {
    let passwords: Vec<Password> = input
        .iter()
        .map(|x| x.parse::<Password>().unwrap())
        .collect();

    passwords
        .iter()
        .filter(|pass| pass.is_valid_part_two())
        .count()
}

#[test]
fn part1_example_1() {
    let v = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(2, part_one(&v));
}

#[test]
fn part2_example_1() {
    let v = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(1, part_two(&v));
}
