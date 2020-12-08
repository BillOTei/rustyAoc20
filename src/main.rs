extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;

use pest::Parser;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

fn main() {
    println!("{}", solve_step_71().unwrap());
}

#[derive(Parser)]
#[grammar = "./day_07/07.pest"]
struct BagsParser;

const INPUT: &str = include_str!("./day_07/07_in");

fn solve_step_71() -> Option<i32> {
    let file = BagsParser::parse(Rule::input, INPUT)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    let mut bags: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

    for bag in file.into_inner() {
        match bag.as_rule() {
            Rule::bag_def => {
                let mut bag_color = "";
                let mut sub_bags: HashMap<&str, usize> = HashMap::new();
                for bag_data in bag.into_inner() {
                    match bag_data.as_rule() {
                        Rule::bag_name => bag_color = bag_data.as_str(),
                        Rule::bag_empty => (),
                        Rule::bags_list => {
                            let mut inner_bags = bag_data.into_inner();
                            let number: usize = inner_bags.next()?.as_str().parse().ok()?;
                            let bag_name = inner_bags.next()?.as_str();
                            sub_bags.insert(bag_name, number);
                        }
                        _ => unreachable!(),
                    }
                }
                bags.insert(bag_color, sub_bags);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    for (key, value) in &bags {
        println!("{}", key);

        for (key2, value) in bags.get(key).unwrap() {
            println!("{} {}", key2, value);
        }
    }

    Some(12)
}

fn recurse_bags(bags: &HashMap<&str, HashMap<&str, usize>>, color: &str) {
    ()
}

