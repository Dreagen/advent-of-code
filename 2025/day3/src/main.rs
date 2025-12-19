use std::{fs, time::Instant};

fn main() {
    let start = Instant::now();
    part2();
    let duration = start.elapsed();
    println!("\nDuration: {:?}", duration);
}

pub fn part1() {
    let input = fs::read_to_string("input.txt").expect("couldn't find input");
    // let input = fs::read_to_string("test.txt").expect("couldn't find input");
    let banks = input.split_whitespace();

    let mut total = 0;
    for bank in banks {
        let mut index_of_highest = 0;
        let mut highest = 0;
        let mut next_highest = 0;

        let bank_digits = bank
            .chars()
            .map(|c| c.to_digit(10).expect("wasn't a number"))
            .collect::<Vec<_>>();

        // println!("Bank: {:?}", bank_digits);

        for (i, num) in bank_digits.iter().enumerate() {
            if i == bank.len() - 1 {
                continue;
            }

            if *num > highest {
                highest = *num;
                index_of_highest = i;
            }
        }

        for (i, num) in bank_digits[index_of_highest + 1..].iter().enumerate() {
            if i == bank.len() - 1 {
                continue;
            }

            if *num > next_highest {
                next_highest = *num;
            }
        }

        // println!("Highest output: {}{}", highest, next_highest);

        total += format!("{}{}", highest, next_highest)
            .parse::<u64>()
            .expect("could not join numbers");
    }

    println!("\n\nResult: {}", total);
}

pub fn part2() {
    let input = fs::read_to_string("input.txt").expect("couldn't find input");
    // let input = fs::read_to_string("test.txt").expect("couldn't find input");
    let banks = input.split_whitespace();

    let mut total = 0;

    for bank in banks {
        let bank_digits = bank
            .trim()
            .chars()
            .map(|c| c.to_digit(10).expect("wasn't a number"))
            .collect::<Vec<_>>();

        let bank_digits_len = bank_digits.len();

        // println!("Bank: {:?}", bank_digits);

        let mut number = "".to_string();

        for (i, bank_num) in bank_digits.iter().enumerate() {
            let remaining_in_bank = bank_digits_len - i;
            let remaining_to_fill_battery = 12 - number.len();

            if number.len() == 0 {
                number = bank_num.to_string();
                continue;
            }

            let bank_num_char = format!("{}", *bank_num);

            let remaining_difference = remaining_in_bank - remaining_to_fill_battery;
            if remaining_difference == 0 {
                number = number + &bank_num_char;
                continue;
            }

            let mut amount_to_skip = 0;
            if number.len() > remaining_difference {
                amount_to_skip = number.len() - remaining_difference;
            }

            let mut replace_from_index: i32 = -1;
            for (index, num) in number
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect(format!("[ {} ] wasn't a number", c).as_ref())
                })
                .enumerate()
                .skip(amount_to_skip)
            {
                if bank_num > &num {
                    replace_from_index = i32::try_from(index).unwrap();
                    break;
                }
            }

            if replace_from_index == -1 {
                if remaining_to_fill_battery > 0 {
                    number = number + &bank_num_char;
                }
            } else {
                number = replace_from_starting_index(&number, bank_num_char, replace_from_index);
            }
        }

        // println!("\nlargest battery: {}\n", number);

        total += number.parse::<u64>().expect("largest battery not a number");
    }

    println!("\n\nResult: {}", total);
}

fn replace_from_starting_index(s: &str, new_char: String, index: i32) -> String {
    let prefix: String = s.chars().take(usize::try_from(index).unwrap()).collect();
    format!("{}{}", prefix, new_char)
}
