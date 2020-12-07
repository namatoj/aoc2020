use std::collections::{HashMap, HashSet};

pub fn run(input: Vec<String>) {
    println!("Day 7");
    println!("=====");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

pub fn part_one(input: &[String]) -> u32 {
    let mut rules: HashMap<String, String> = HashMap::new();

    for rule in input {
        let temp = rule.replace("bags", "");
        let temp = temp.replace("bag", "");
        let mut temp2 = temp.split("contain");
        rules.insert(
            temp2.next().unwrap().trim().to_string(),
            temp2.next().unwrap().trim().to_string(),
        );
    }

    let mut queue: Vec<String> = vec!["shiny gold".to_string()];

    let mut possible_outer_bags: HashSet<String> = HashSet::new();

    while !queue.is_empty() {
        let current_bag = queue.pop().unwrap();

        for (key, val) in &rules {
            if val.contains(&current_bag) {
                queue.push(key.clone());
                possible_outer_bags.insert(key.to_string());
            }
        }
    }

    possible_outer_bags.len() as u32
}

fn calculate_bags(rule_name: &str, all_rules: &[String]) -> u32 {
    let mut rule = None;
    for rule_string in all_rules {
        let temp: Vec<_> = rule_string.split("contain").collect();
        if temp[0].contains(rule_name) {
            rule = Some(temp[1].trim().strip_suffix('.').unwrap());
            break;
        }
    }

    let mut bags = 1;
    let split_rule: Vec<_> = rule.unwrap().split(',').collect();

    if !split_rule[0].contains("no other bags") {
        for bag_rule in split_rule {
            let x: Vec<_> = bag_rule.trim().splitn(2, ' ').collect();
            let multiplier: u32 = x[0].parse().unwrap();
            bags += multiplier * calculate_bags(&x[1].to_string(), &all_rules);
        }
    }
    bags
}

pub fn part_two(input: &[String]) -> u32 {
    let start_bag = "shiny gold bag";

    calculate_bags(&start_bag.to_string(), &input) - 1
}

#[test]
fn part1_example_1() {
    let v = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(4, part_one(&v));
}

#[test]
fn part2_example_1() {
    let v = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(32, part_two(&v));
}

#[test]
fn part2_example_2() {
    let v = vec![
        "shiny gold bags contain 2 dark red bags.",
        "dark red bags contain 2 dark orange bags.",
        "dark orange bags contain 2 dark yellow bags.",
        "dark yellow bags contain 2 dark green bags.",
        "dark green bags contain 2 dark blue bags.",
        "dark blue bags contain 2 dark violet bags.",
        "dark violet bags contain no other bags.",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(126, part_two(&v));
}
