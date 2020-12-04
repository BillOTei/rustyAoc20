use regex::{Regex, RegexSet};

const INPUT: &str = include_str!("./04_in");
const HEADERS: &[&str; 7] = &[
    r"byr:",
    r"iyr:",
    r"eyr:",
    r"hgt:",
    r"hcl:",
    r"ecl:",
    r"pid:"
];
const HEADERS_LEN: &usize = &HEADERS.len();

pub fn solve_step_1() -> usize {
    let re = Regex::new(r"\n\n").expect("Invalid regex");
    let set = RegexSet::new(HEADERS).expect("Invalid regex set");

    re
        .split(INPUT)
        .filter(|l| {
            let check: Vec<usize> = set.matches(l).into_iter().collect();

            &check.len() == HEADERS_LEN
        })
        .count()
}

fn is_valid_byr(l: &str) -> bool {
    let re_byr = Regex::new(r"byr:(?P<byr>\d{4})").expect("Invalid regex byr");

    match re_byr
        .captures(l)
        .and_then(|c| c.name("byr")) {
        Some(y) => {
            let year: usize = y.as_str().parse().unwrap();
            year >= 1920 && year <= 2002
        }
        _ => false
    }
}

fn is_valid_iyr(l: &str) -> bool {
    let re_iyr = Regex::new(r"iyr:(?P<iyr>\d{4})").expect("Invalid regex iyr");

    match re_iyr
        .captures(l)
        .and_then(|c| c.name("iyr")) {
        Some(y) => {
            let year: usize = y.as_str().parse().unwrap();
            year >= 2010 && year <= 2020
        }
        _ => false
    }
}

fn is_valid_eyr(l: &str) -> bool {
    let re_eyr = Regex::new(r"eyr:(?P<eyr>\d{4})").expect("Invalid regex eyr");

    match re_eyr
        .captures(l)
        .and_then(|c| c.name("eyr")) {
        Some(y) => {
            let year: usize = y.as_str().parse().unwrap();
            year >= 2020 && year <= 2030
        }
        _ => false
    }
}

fn is_valid_hgt(l: &str) -> bool {
    let re_hgt_cm = Regex::new(r"hgt:(?P<hgtcm>\d{3})cm").expect("Invalid regex hgt");
    let re_hgt_in = Regex::new(r"hgt:(?P<hgtin>\d{2})in").expect("Invalid regex hgt");

    match re_hgt_cm
        .captures(l)
        .and_then(|c| c.name("hgtcm")) {
        Some(h) => {
            let height: usize = h.as_str().parse().unwrap();
            height >= 150 && height <= 193
        }
        _ => match re_hgt_in
            .captures(l)
            .and_then(|c| c.name("hgtin")) {
            Some(h) => {
                let height: usize = h.as_str().parse().unwrap();
                height >= 59 && height <= 76
            }
            _ => false
        }
    }
}

fn is_valid_hcl(l: &str) -> bool {
    let re_hcl = Regex::new(r"hcl:(?P<hcl>#[a-f0-9]{6})").expect("Invalid regex hcl");

    match re_hcl
        .captures(l)
        .and_then(|c| c.name("hcl")) {
        Some(_) => true,
        _ => false
    }
}

fn is_valid_ecl(l: &str) -> bool {
    let re_ecl = Regex::new(r"ecl:(?P<ecl>amb|blu|brn|gry|grn|hzl|oth)").expect("Invalid regex ecl");

    match re_ecl
        .captures(l)
        .and_then(|c| c.name("ecl")) {
        Some(_) => true,
        _ => false
    }
}

fn is_valid_pid(l: &str) -> bool {
    match Regex::new(r"pid:(\W?\w+)").unwrap()
        .captures(l) {
        Some(_pid) => {
            let v = _pid.get(1).unwrap().as_str();
            v.len() == 9 && Regex::new(r"[^0-9]").unwrap().captures(v).is_none()
        }
        None => false
    }
}

pub fn solve_step_2() -> usize {
    let re = Regex::new(r"\n\n").expect("Invalid regex");
    let set = RegexSet::new(HEADERS).expect("Invalid regex set");

    re
        .split(INPUT)
        .filter(|l| {
            let check: Vec<usize> = set.matches(l).into_iter().collect();

            if &check.len() == HEADERS_LEN {
                is_valid_byr(&l) && is_valid_iyr(&l) && is_valid_eyr(&l) && is_valid_hgt(&l) && is_valid_hcl(&l) &&
                    is_valid_ecl(&l) && is_valid_pid(&l)
            } else {
                false
            }
        })
        .count()
}
