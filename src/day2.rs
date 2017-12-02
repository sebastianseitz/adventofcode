use std::io::BufReader;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn get_reader(filename: &str) -> Result<Vec<String>, io::Error> {
    println!("Reading {}", filename);

    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);

    return file.lines().collect()
}

fn part_one(filename: &str) -> u32 {
    let file = get_reader(filename);

    let mut sum = 0;
    for line in file.unwrap() {
        let mut min = u32::max_value();
        let mut max = u32::min_value();
        for candidate in line.split_whitespace() {
            let y: u32 = candidate.parse().expect("NaN");

            if y < min {
                min = y;
            }

            if y > max {
                max = y;
            }
        }

        sum += max - min;
    }

    return sum
}

fn part_two(filename: &str) -> u32 {
    let file = get_reader(filename);

    let mut sum = 0;
    for line in file.unwrap() {
        let mut numbers: Vec<u32> = vec![];
        for candidate in line.split_whitespace() {
            numbers.push(candidate.parse().expect("NaN"));
        }
        for idx in 0..numbers.len() {
            let nr_1 = numbers[idx];

            if idx == numbers.len() - 1 {
                continue
            }

            for idx_2 in idx+1..numbers.len() {
                let nr_2 = numbers[idx_2];

                if nr_1 % nr_2 == 0 {
                    sum += nr_1 / nr_2;
                } else if nr_2 % nr_1 == 0 {
                    sum += nr_2 / nr_1;
                }
            }
        }
    }

    return sum
}

fn main() {
    let result_one = part_one(
        "/Users/basti/Development/adventofcode/inputs/day2.input"
    );

    println!("Result One: {}", result_one);

    let result_two = part_two(
        "/Users/basti/Development/adventofcode/inputs/day2.input"
    );

    println!("Result Two: {}", result_two);
}
