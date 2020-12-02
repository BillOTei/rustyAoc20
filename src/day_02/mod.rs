use std::{fs::File, io::{BufReader, prelude::*}, path::Path, env};

extern crate serde_scan;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_line(line: &str) -> (usize, usize, char, String) {
    serde_scan::scan!("{}-{} {}: {}" <- line).unwrap()
}

fn is_valid(line: &str) -> bool {
    let (min, max, letter, password) = parse_line(line);
    let count = password.matches(letter).count();

    min <= count && count <= max
}

fn is_valid2(line: &str) -> bool {
    let (min, max, letter, password) = parse_line(line);
    let char_at_min = match password.chars().nth(min - 1) {
        Some(ch) => ch == letter,
        None => false
    };
    let char_at_max = match password.chars().nth(max - 1) {
        Some(ch) => ch == letter,
        None => false
    };

    (char_at_min && !char_at_max) || (!char_at_min && char_at_max)
}

pub fn solve_step_1() -> usize {
    let path = env::current_dir().expect("can't read current dir");

    lines_from_file(format!("{}{}", path.display(), "/src/day_02/02_in"))
        .into_iter()
        .filter(|l| is_valid(l))
        .count()
}

pub fn solve_step_2() -> usize {
    let path = env::current_dir().expect("can't read current dir");

    lines_from_file(format!("{}{}", path.display(), "/src/day_02/02_in"))
        .into_iter()
        .filter(|l| is_valid2(l))
        .count()
}