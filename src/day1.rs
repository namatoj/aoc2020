pub fn run(input: Vec<String>) {
    println!("Day 1");
    println!("=====");
    // println!("{:?}", input);

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

pub fn part_one(input: &[String]) -> i32 {
    let values: Vec<i32> = input.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    for x in &values {
        for y in &values {
            let sum = x + y;
            if sum == 2020 {
                return x * y;
            }
        }
    }
    panic!("No sum of 2020 in input");
}

pub fn part_two(input: &[String]) -> i32 {
    let values: Vec<i32> = input.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    for x in &values {
        for y in &values {
            for z in &values {
                let sum = x + y + z;
                if sum == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    panic!("No three numbers sum to 2020 in input");
}

#[test]
fn part1_example_1() {
    let v = vec!["1721", "979", "366", "299", "675", "1456"];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();

    assert_eq!(514579, part_one(&v));
}

#[test]
fn part2_example_1() {
    let v = vec!["1721", "979", "366", "299", "675", "1456"];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();

    assert_eq!(241861950, part_two(&v));
}
