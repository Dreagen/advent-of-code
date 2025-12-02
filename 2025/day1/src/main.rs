use std::fs;

fn main() {
    let part1 = fs::read_to_string("part-1.txt").expect("couldn't find part-1.txt");
    let instructions = part1.split_whitespace().map(|item| Instruction::new(item));

    let mut count_of_zeros = 0;
    let mut dial = Dial { counter: 50 };

    for instruction in instructions {
        match instruction.direction {
            Direction::Left => dial.decrement(instruction.amount),
            Direction::Right => dial.increment(instruction.amount),
        }

        if dial.counter == 0 {
            count_of_zeros += 1;
        }
    }

    println!("Result: {}", count_of_zeros);
}

struct Dial {
    counter: i32,
}

impl Dial {
    fn increment(&mut self, amount: i32) {
        self.counter = (self.counter + amount).rem_euclid(100);
    }

    fn decrement(&mut self, amount: i32) {
        self.counter = (self.counter - amount).rem_euclid(100);
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
