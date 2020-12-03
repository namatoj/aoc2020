#[derive(Debug)]
struct Slope {
    right: usize,
    down: usize,
}

pub fn run(input: Vec<String>) {
    println!("Day 3");
    println!("=====");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

fn check_slope(input: &[String], slope: &Slope) -> u64 {
    let mut trees_hit = 0;
    let mut column = 0;
    for line in input.iter().step_by(slope.down) {
        if line.chars().nth(column) == Some('#') {
            trees_hit += 1;
        }

        column += slope.right;
        let characters = line.chars().count();
        if column >= characters {
            column -= characters;
        }
    }
    trees_hit
}

pub fn part_one(input: &[String]) -> u64 {
    check_slope(&input, &Slope { right: 3, down: 1 })
}

pub fn part_two(input: &[String]) -> u64 {
    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    let mut result = 1;
    for slope in slopes {
        result *= check_slope(input, &slope);
    }
    result
}

#[test]
fn part1_example_1() {
    let v = vec![
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();

    assert_eq!(7, part_one(&v));
}

#[test]
fn part2_example_1() {
    let v = vec![
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();

    assert_eq!(336, part_two(&v));
}
