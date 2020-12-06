use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    println!("Day 6");
    println!("=====");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

pub fn part_one(input: &[String]) -> u32 {
    let input = input.join("\n");
    let input = input.split("\n\n");

    let mut result = 0;

    let mut answer_set: HashSet<char> = HashSet::new();
    for group_answer in input {
        let person_answers = group_answer.split('\n');
        for person_answer in person_answers {
            for c in person_answer.chars() {
                answer_set.insert(c);
            }
        }

        result += answer_set.iter().count();
        answer_set.clear();
    }

    result as u32
}

pub fn part_two(input: &[String]) -> u32 {
    let input = input.join("\n");
    let input = input.split("\n\n");

    let mut result = 0;

    for group_answer in input {
        let mut person_answers: Vec<String> =
            group_answer.split('\n').map(|s| s.to_string()).collect();
        let mut all_answered = person_answers.pop().unwrap();

        for answer in &person_answers {
            let temp_all_answered = all_answered.clone();
            let answer_chars = temp_all_answered.chars();
            for answer_char in answer_chars {
                // if char in all_answered is not in answer, remove it from all answered.
                if !answer.contains(answer_char) {
                    all_answered = all_answered.replace(answer_char, "");
                }
            }
        }

        result += all_answered.len();
    }

    result as u32
}

#[test]
fn part1_example_1() {
    let v = vec![
        "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(11, part_one(&v));
}

#[test]
fn part2_example_1() {
    let v = vec![
        "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(6, part_two(&v));
}
