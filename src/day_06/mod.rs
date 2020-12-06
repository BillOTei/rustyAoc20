use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("06_in");

pub fn solve_step_1() -> usize {
    let re = Regex::new(r"\n\n").expect("Invalid regex");

    re
        .split(INPUT)
        .map(|l| questions_map(l).len())
        .sum()
}

fn questions_map(l: &str) -> HashSet<char> {
    l
        .chars()
        .filter(|c| c != &'\n')
        .fold(HashSet::new(), |mut hs: HashSet<char>, p| {
            hs.insert(p);
            hs
        })
}

pub fn solve_step_2() -> usize {
    let re = Regex::new(r"\n\n").expect("Invalid regex");
    let re_l = Regex::new(r"\n").expect("Invalid regex");

    re
        .split(INPUT)
        .map(|l| {
            let questions = questions_map(l);
            let questions_count = l
                .chars()
                .filter(|c| c != &'\n')
                .fold(HashMap::new(), |mut hs: HashMap<char, usize>, p| {
                    hs.insert(p, hs.get(&p).unwrap_or(&0usize) + 1);
                    hs
                });
            let persons_count = re_l
                .split(l)
                .collect_vec()
                .len();

            questions
                .iter()
                .filter(|q| questions_count.get(q).unwrap_or(&0usize) == &persons_count)
                .collect_vec()
                .len()
        })
        .sum()
}