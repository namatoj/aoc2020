pub fn run(input: Vec<String>) {
    println!("Day 8");
    println!("=====");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

#[derive(Debug, Clone)]
enum Operation {
    Accumulator,
    Jump,
    NoOperation,
}

#[derive(Debug, Clone)]
struct Instruction {
    operation: Operation,
    argument: i32,
    has_executed: bool,
}

impl Instruction {
    fn new(input: &String) -> Instruction {
        let mut iter = input.split_whitespace();
        let operation = match iter.next().unwrap() {
            "acc" => Operation::Accumulator,
            "jmp" => Operation::Jump,
            "nop" => Operation::NoOperation,
            _ => panic!("operator in program not recognized."),
        };

        let argument = iter.next().unwrap().parse::<i32>().unwrap();

        let has_executed = false;
        Instruction {
            operation,
            argument,
            has_executed,
        }
    }
}

#[derive(Debug)]
struct Computer {
    program: Vec<Instruction>,
    program_counter: i32,
    accumulator: i32,
    terminated_correctly: bool,
}

impl Computer {
    fn new(program: Vec<Instruction>) -> Computer {
        Computer {
            program,
            program_counter: 0,
            accumulator: 0,
            terminated_correctly: false,
        }
    }

    fn run(&mut self) -> i32 {
        loop {
            if self.program_counter == self.program.len() as i32 {
                self.terminated_correctly = true;
                break;
            }

            if (self.program_counter >= self.program.len() as i32) || (self.program_counter < 0) {
                break;
            }

            let mut current_instruction = &mut self.program[self.program_counter as usize];

            if current_instruction.has_executed {
                break;
            }

            match current_instruction.operation {
                Operation::Accumulator => {
                    self.accumulator += current_instruction.argument;
                    self.program_counter += 1;
                }
                Operation::Jump => {
                    self.program_counter += current_instruction.argument;
                }
                Operation::NoOperation => {
                    self.program_counter += 1;
                }
            }

            current_instruction.has_executed = true;
        }

        self.accumulator
    }
}

pub fn part_one(input: &[String]) -> i32 {
    let mut program: Vec<Instruction> = Vec::new();
    for line in input {
        program.push(Instruction::new(line));
    }

    let mut computer = Computer::new(program);

    computer.run();

    computer.accumulator
}

pub fn part_two(input: &[String]) -> i32 {
    let mut program: Vec<Instruction> = Vec::new();
    for line in input {
        program.push(Instruction::new(line));
    }

    for i in 0..program.len() {
        let mut program_clone = program.clone();

        match program_clone[i].operation {
            Operation::Accumulator => continue,
            Operation::Jump => program_clone[i].operation = Operation::NoOperation,
            Operation::NoOperation => program_clone[i].operation = Operation::Jump,
        }

        let mut computer = Computer::new(program_clone);
        computer.run();
        if computer.terminated_correctly {
            return computer.accumulator;
        }
    }

    panic!("computer never terminated correctly");
}

#[test]
fn part1_example_1() {
    let v = vec![
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(5, part_one(&v));
}

#[test]
fn part2_example_1() {
    let v = vec![
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();
    assert_eq!(8, part_two(&v));
}
