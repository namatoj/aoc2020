use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    println!("Day 5");
    println!("=====");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

fn calculate_seat_id(seat: &str) -> u32 {
    let (row, col) = seat.split_at(7);

    let mut row_value = 0;

    for (i, c) in row.chars().enumerate() {
        if c == 'B' {
            row_value += 2u32.pow(6 - i as u32);
        }
    }

    let mut col_value = 0;
    for (i, c) in col.chars().enumerate() {
        if c == 'R' {
            col_value += 2u32.pow(2 - i as u32);
        }
    }

    row_value * 8 + col_value
}

pub fn part_one(input: &[String]) -> u32 {
    input
        .iter()
        .map(|row| calculate_seat_id(&row))
        .max()
        .unwrap()
}

pub fn part_two(input: &[String]) -> u32 {
    let found_ids: HashSet<_> = input.iter().map(|row| calculate_seat_id(&row)).collect();
    let min_id: u32 = *found_ids.iter().min().unwrap();
    let max_id: u32 = *found_ids.iter().max().unwrap();

    let all_ids: HashSet<u32> = (min_id..max_id + 1).collect();

    let mut diff = all_ids.difference(&found_ids);

    *diff.next().unwrap()
}

#[test]
fn part1_example_1() {
    let v = vec!["BFFFBBFRRR"];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(567, part_one(&v));
}

#[test]
fn part1_example_2() {
    let v = vec!["FFFBBBFRRR"];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(119, part_one(&v));
}

#[test]
fn part1_example_3() {
    let v = vec!["BBFFBBFRLL"];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(820, part_one(&v));
}
