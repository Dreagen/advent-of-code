use std::{fs, time::Instant};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let start = Instant::now();
    part1();
    let duration = start.elapsed();
    println!("\nDuration: {:?}", duration);
}

pub fn part1() {
    let input = fs::read_to_string("input.txt").expect("couldn't find input");
    // let input = fs::read_to_string("test.txt").expect("couldn't find input");
    let rows = input.split_whitespace();

    let mut grid: Vec<Vec<char>> = vec![];

    for row in rows {
        let chars_in_row = row.chars().collect::<Vec<_>>();
        grid.push(chars_in_row);
    }

    let mut y_index = 0;
    let mut count = 0;
    for row in &grid {
        // println!("{:?}", row);

        let mut x_index = 0;
        for _ in row {
            if grid[y_index][x_index] != '@' {
                x_index += 1;
                continue;
            }

            if count_surrounding(&grid, y_index, x_index) < 4 {
                count += 1;
            }

            x_index += 1;
        }
        y_index += 1;
    }

    println!("Result: {}", count);
}

fn count_surrounding(grid: &Vec<Vec<char>>, y_index: usize, x_index: usize) -> u32 {
    let y_max = grid.len() - 1;
    let x_max = grid.last().unwrap().len() - 1;

    let mut count = 0;
    for position in Position::iter() {
        count += match position {
            Position::Left => {
                if x_index == 0 {
                    0
                } else {
                    let value = grid[y_index][x_index - 1];
                    match value {
                        '@' => 1,
                        _ => 0,
                    }
                }
            }
            Position::TopLeft => {
                if x_index == 0 || y_index == 0 {
                    0
                } else {
                    let value = grid[y_index - 1][x_index - 1];
                    match value {
                        '@' => 1,
                        _ => 0,
                    }
                }
            }
            Position::Top => {
                if y_index == 0 {
                    0
                } else {
                    let value = grid[y_index - 1][x_index];
                    match value {
                        '@' => 1,
                        _ => 0,
                    }
                }
            }
            Position::TopRight => {
                if x_index == x_max || y_index == 0 {
                    0
                } else {
                    let value = grid[y_index - 1][x_index + 1];
                    match value {
                        '@' => 1,
                        _ => 0,
                    }
                }
            }
            Position::Right => {
                if x_index == x_max {
                    0
                } else {
                    let value = grid[y_index][x_index + 1];
                    match value {
                        '@' => 1,
                        _ => 0,
                    }
                }
            }
            Position::BottomRight => {
                if x_index == x_max || y_index == y_max {
                    0
                } else {
                    let value = grid[y_index + 1][x_index + 1];
                    match value {
                        '@' => 1,
                        _ => 0,
                    }
                }
            }
            Position::Bottom => {
                if y_index == y_max {
                    0
                } else {
                    let value = grid[y_index + 1][x_index];
                    match value {
                        '@' => 1,
                        _ => 0,
                    }
                }
            }
            Position::BottomLeft => {
                if x_index == 0 || y_index == y_max {
                    0
                } else {
                    let value = grid[y_index + 1][x_index - 1];
                    match value {
                        '@' => 1,
                        _ => 0,
                    }
                }
            }
        };
    }

    count
}

#[derive(Debug, EnumIter)]
enum Position {
    Left,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
}
