use std::collections::HashSet;

use std::env;

mod util;

fn day_six(banks: String) {
    let mut real: Vec<u32> = vec![];
    let mut lookup: HashSet<Vec<u32>> = HashSet::new();

    for bank in banks.split_whitespace() {
        real.push(bank.parse().expect("NaN"));
    }

    lookup.insert(real.clone());

    let mut counter = 0;
    let mut seen = false;
    let mut search: Vec<u32> = vec![];

    loop {

        let mut cur_max = std::u32::MIN;
        let mut cur_idx = 0;

        for idx in 0..real.len() {
            if real[idx] > cur_max {
                cur_max = real[idx];
                cur_idx = idx;
            }

            if real[idx] == cur_max {
                cur_idx = std::cmp::min(idx, cur_idx);
            }
        }

        let mut redistributable = real[cur_idx];
        let mut stepper = cur_idx + 1;
        real[cur_idx] = 0;

        while redistributable > 0 {
            if stepper >= real.len() {
                stepper = 0;
            }

            real[stepper] += 1;
            stepper += 1;
            redistributable -= 1;
        }

        counter += 1;

        if seen {
            if search == real {
                println!("Second cycles: {}", counter);
                break;
            }
        } else {
            if lookup.contains(&real) {
                println!("First cycles: {}", counter);
                seen = true;
                counter = 0;
                search = real.clone();
            } else {
                lookup.insert(real.clone());
            }
        }

    }
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let lines = util::read_lines(filename);

    day_six(lines[0].clone());
}
