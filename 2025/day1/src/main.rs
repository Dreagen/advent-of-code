use std::fs;

fn main() {
    part2();
}

fn part1() {
    // let input = fs::read_to_string("input.txt").expect("couldn't find part-1.txt");
    let input = fs::read_to_string("test.txt").expect("couldn't find part-1.txt");
    let instructions = input.split_whitespace().map(|item| Instruction::new(item));

    let mut count_of_zeros = 0;
    let mut dial = Dial { counter: 50 };

    for instruction in instructions {
        println!("\nInstruction: {:?}", instruction);
        _ = match instruction.direction {
            Direction::Left => dial.decrement(instruction.amount),
            Direction::Right => dial.increment(instruction.amount),
        };

        if dial.counter == 0 {
            count_of_zeros += 1;
        }
    }

    println!("Result: {}", count_of_zeros);
}

fn part2() {
    let input = fs::read_to_string("input.txt").expect("couldn't find part-1.txt");
    // let input = fs::read_to_string("test.txt").expect("couldn't find part-1.txt");
    let instructions = input.split_whitespace().map(|item| Instruction::new(item));

    let mut count_of_zeros = 0;
    let mut dial = Dial { counter: 50 };

    for instruction in instructions {
        println!("\nInstruction: {:?}", instruction);
        count_of_zeros += match instruction.direction {
            Direction::Left => dial.decrement(instruction.amount),
            Direction::Right => dial.increment(instruction.amount),
        };

        println!("Count of Zeros: {}", count_of_zeros);
    }

    println!("Result: {}", count_of_zeros);
}

struct Dial {
    counter: i32,
}

impl Dial {
    fn increment(&mut self, mut amount: i32) -> i32 {
        let mut zeros_touched = 0;

        loop {
            if amount == 0 {
                break;
            }

            if self.counter == 99 {
                self.counter = 0;
                amount -= 1;
            } else {
                self.counter += 1;
                amount -= 1;
            }

            if self.counter == 0 {
                zeros_touched += 1;
            }
        }

        zeros_touched
    }

    fn decrement(&mut self, mut amount: i32) -> i32 {
        let mut zeros_touched = 0;

        loop {
            if amount == 0 {
                break;
            }

            if self.counter == 0 {
                self.counter = 99;
                amount -= 1;
            } else {
                self.counter -= 1;
                amount -= 1;
            }

            if self.counter == 0 {
                zeros_touched += 1;
            }
        }

        zeros_touched
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: i32,
}

impl Instruction {
    fn new(input: &str) -> Instruction {
        let direction = match &input[..1] {
            "R" => Direction::Right,
            _ => Direction::Left,
        };
        let amount = input[1..].parse::<i32>().expect("amount wasn't an int");

        Instruction { direction, amount }
    }
}
