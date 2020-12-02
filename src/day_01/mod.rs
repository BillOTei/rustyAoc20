use std::{env, fs};
use std::num::ParseIntError;

fn get(first_number_str: &str, second_number_str: &str) -> Result<(i32, i32), ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| (first_number, second_number))
    })
}

fn get2(first_number_str: &str, second_number_str: &str, third_number_str: &str) -> Result<(i32, i32, i32), ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().and_then(|second_number| {
            third_number_str.parse::<i32>().map(|third_number| (first_number, second_number, third_number))
        })
    })
}

pub fn solve_step_1() -> i32 {
    let path = env::current_dir().expect("can't read current dir");
    let contents = fs::read_to_string(format!("{}{}", path.display(), "/src/day_01/01_in"))
        .expect("Something went wrong reading the file");
    let expenses = contents.split("\n").collect::<Vec<&str>>();
    let mut result: i32 = 0;

    'outer: for i in 0..(expenses.len() - 1) {
        for j in i..(expenses.len() - 1) {
            match get(expenses[i], expenses[j]) {
                Ok((x, y)) if x + y == 2020 => {
                    result = x * y;
                    break 'outer;
                }
                _ => result = -1
            }
        }
    }

    result
}

pub fn solve_step_2() -> i32 {
    let path = env::current_dir().expect("can't read current dir");
    let contents = fs::read_to_string(format!("{}{}", path.display(), "/src/day_01/01_in"))
        .expect("Something went wrong reading the file");
    let expenses = contents.split("\n").collect::<Vec<&str>>();
    let mut result2: i32 = 0;

    'outer2: for i in 0..(expenses.len() - 1) {
        for j in i..(expenses.len() - 1) {
            for k in j..(expenses.len() - 1) {
                match get2(expenses[i], expenses[j], expenses[k]) {
                    Ok((x, y, z)) if x + y + z == 2020 => {
                        result2 = x * y * z;
                        break 'outer2;
                    }
                    _ => result2 = -1
                }
            }
        }
    }

    result2
}