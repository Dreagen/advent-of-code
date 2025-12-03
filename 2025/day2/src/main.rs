use std::{fs, time::Instant};

fn main() {
    let start = Instant::now();
    part2();
    let duration = start.elapsed();
    println!("\nDuration: {:?}", duration);
}

pub fn part1() {
    // let input = fs::read_to_string("input.txt").expect("couldn't find input");
    let input = fs::read_to_string("test.txt").expect("couldn't find input");
    let ranges = input.split(',');

    let mut total = 0;

    for range in ranges {
        let mut split = range.split('-');

        let lower = split
            .next()
            .expect("didn't find lower part of range")
            .trim()
            .parse::<i64>()
            .expect("lower wasn't a number");

        let higher = split
            .next()
            .expect("didn't find higher part of range")
            .trim()
            .parse::<i64>()
            .expect("higher wasn't a number");

        assert!(lower <= higher, "lower was higher than higher");

        for i in lower..=higher {
            let string_i = i.to_string();

            let length = string_i.len();

            if length % 2 != 0 {
                continue;
            }

            let half_length = length / 2;

            let left = string_i[..half_length].to_string();
            let right = string_i[half_length..].to_string();

            if left == right {
                total += i;
            }
        }
    }

    println!("\n\nResult: {}", total);
}

pub fn part2() {
    let input = fs::read_to_string("input.txt").expect("couldn't find input");
    // let input = fs::read_to_string("test.txt").expect("couldn't find input");
    let ranges = input.split(',');

    let mut total = 0;

    for range in ranges {
        let mut split = range.split('-');

        let lower = split
            .next()
            .expect("didn't find lower part of range")
            .trim()
            .parse::<i64>()
            .expect("lower wasn't a number");

        let higher = split
            .next()
            .expect("didn't find higher part of range")
            .trim()
            .parse::<i64>()
            .expect("higher wasn't a number");

        assert!(lower <= higher, "lower was higher than higher");

        for i in lower..=higher {
            if is_number_repeating(i) {
                total += i;
            }
        }
    }

    println!("\n\nResult: {}", total);
}

fn is_number_repeating(input: i64) -> bool {
    let input_string = input.to_string();

    if input_string.len() < 2 {
        return false;
    }

    let half_length = input_string.len() / 2;

    let mut i = half_length;
    loop {
        if i == 0 {
            break;
        }

        if input_string.len() % i != 0 {
            i -= 1;
            continue;
        }

        let chunks = chunk(&input_string, i);
        let first_chunk = chunks.iter().next().unwrap();
        if chunks.iter().all(|chunk| chunk == first_chunk) {
            return true;
        }

        i -= 1;
    }

    false
}

fn chunk(input: &str, size: usize) -> Vec<String> {
    input
        .chars()
        .collect::<Vec<_>>()
        .chunks(size)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
}
