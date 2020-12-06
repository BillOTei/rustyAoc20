const INPUT: &str = include_str!("05_in");

fn front(low_y: usize, hi_y: usize) -> (usize, usize) {
    //println!("{} {}", low_y, hi_y);

    (low_y, low_y + (hi_y - low_y) / 2)
}

fn back(low_y: usize, hi_y: usize) -> (usize, usize) {
    //println!("{} {}", low_y, hi_y);

    (low_y + (hi_y - low_y) / 2 + 1, hi_y)
}

fn left(low_x: usize, hi_x: usize) -> (usize, usize) {
    //println!("{} {}", low_x, hi_x);

    (low_x, low_x + (hi_x - low_x) / 2)
}

fn right(low_x: usize, hi_x: usize) -> (usize, usize) {
    //println!("{} {}", low_x, hi_x);

    (low_x + (hi_x - low_x) / 2 + 1, hi_x)
}

fn process_rows(l: &str, low_y: usize, hi_y: usize) -> usize {
    let sub = &l[..7];
    let (y_l, y_h) = sub
        .chars()
        .fold((low_y, hi_y), |acc: (usize, usize), c: char| {
            match c {
                'F' => if hi_y - low_y > 1 {
                    front(acc.0, acc.1)
                } else {
                    (acc.0, acc.1)
                },
                'B' => if hi_y - low_y > 1 {
                    back(acc.0, acc.1)
                } else {
                    (acc.0, acc.1)
                },
                x => panic!(format!("wrong format with {}", x))
            }
        });

    match &sub[6..] {
        "F" => y_l,
        "B" => y_h,
        x => panic!(format!("wrong format with {}", x))
    }
}

fn process_columns(l: &str, low_x: usize, hi_x: usize) -> usize {
    let len = l.len();
    let sub = &l[len - 3..];
    let (x_l, x_h) = sub
        .chars()
        .fold((low_x, hi_x), |acc: (usize, usize), c: char| {
            match c {
                'L' => if hi_x - low_x > 1 {
                    left(acc.0, acc.1)
                } else {
                    (acc.0, acc.1)
                },
                'R' => if hi_x - low_x > 1 {
                    right(acc.0, acc.1)
                } else {
                    (acc.0, acc.1)
                },
                x => panic!(format!("wrong format with {}", x))
            }
        });

    match &sub[2..] {
        "R" => x_l,
        "L" => x_h,
        x => panic!(format!("wrong format with {}", x))
    }
}

pub fn solve_step_1() -> usize {
    let (low_y, hi_y, low_x, hi_x) = (0usize, 127usize, 0usize, 7usize);

    INPUT
        .lines()
        .map(|l| {
            let (last_rows, last_cols) = (process_rows(l, low_y, hi_y), process_columns(l, low_x, hi_x));
            last_rows * 8 + last_cols
        })
        .max()
        .unwrap()
}

pub fn solve_step_2() -> isize {
    let (low_y, hi_y, low_x, hi_x) = (0usize, 127usize, 0usize, 7usize);

    let ids = INPUT
        .lines()
        .map(|l| {
            let (last_rows, last_cols) = (process_rows(l, low_y, hi_y), process_columns(l, low_x, hi_x));
            (last_rows * 8 + last_cols) as isize
        });
    let ids2 = ids.clone();
    let ids3 = ids.clone();
    let min = ids.min().unwrap();
    let max = ids2.max().unwrap();
    let sum: isize = ids3.sum();

    (min..max + 1).sum::<isize>() - sum
}