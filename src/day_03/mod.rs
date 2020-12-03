use std::{env, fs::File, io::{BufReader, prelude::*}, path::Path};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn count_trees(map: impl Iterator<Item=(usize, String)>, slope: (usize, usize)) -> usize {
    let (step_down, step_right) = slope;
    map
        .fold((0, step_right), |(count, x_pos), (i, l)| {
            if i > 0 && i % step_down == 0 {
                let real_x_pos = x_pos % l.len();
                match l.chars().nth(real_x_pos) {
                    Some(c) if c == '#' => (count + 1, x_pos + step_right),
                    _ => (count, x_pos + step_right)
                }
            } else {
                (count, x_pos)
            }
        }).0
}

pub fn solve_step_1() -> usize {
    let path = env::current_dir().expect("can't read current dir");
    let map = lines_from_file(format!("{}{}", path.display(), "/src/day_03/03_in"))
        .into_iter()
        .enumerate();

    count_trees(map, (1, 3))
}

pub fn solve_step_2() -> usize {
    let path = env::current_dir().expect("can't read current dir");
    let map = lines_from_file(format!("{}{}", path.display(), "/src/day_03/03_in"))
        .into_iter()
        .enumerate();

    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .fold(1, |acc, slope| acc * count_trees(map.clone(), *slope))
}