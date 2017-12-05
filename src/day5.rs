use std::env;

mod util;

fn part_one(steps: Vec<String>) -> u32 {
    let mut counter = 0;
    let mut jumps: Vec<i32> = vec![];

    for step in steps {
        jumps.push(step.parse().expect("NaN"));
    }

    let mut idx: usize = 0;
    while idx < jumps.len() {
        let jump = jumps[idx];
        jumps[idx] += 1;

        if jump > 0 {
            idx += jump as usize;
        } else {
            idx -= jump.abs() as usize;
        }

        counter += 1;
    }

    return counter;
}

fn part_two(steps: Vec<String>) -> u32 {
    let mut counter = 0;
    let mut jumps: Vec<i32> = vec![];

    for step in steps {
        jumps.push(step.parse().expect("NaN"));
    }

    let mut idx: usize = 0;
    while idx < jumps.len() {
        let jump = jumps[idx];

        if jump > 2 {
            jumps[idx] -= 1;
        } else {
            jumps[idx] += 1;
        }

        if jump > 0 {
            idx += jump as usize;
        } else {
            idx -= jump.abs() as usize;
        }

        counter += 1;
    }

    return counter;
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let steps = util::read_lines(filename);

    println!("PartOne: {}", part_one(steps.clone()));

    println!("PartTwo: {}", part_two(steps.clone()));
}
