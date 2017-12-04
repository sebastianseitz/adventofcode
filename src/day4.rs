use std::collections::HashSet;
use std::env;

mod util;

fn part_one(phrases: Vec<String>) -> u32 {
    let mut counter = 0;

    for row in phrases {
        let words = row.split_whitespace();
        let mut phrases: HashSet<String> = HashSet::new();
        let mut dirty = false;

        for word in words {
            let cur_word = word.clone();
            if phrases.contains(word) {
                dirty = true;
                break;
            } else {
                phrases.insert(String::from(cur_word));
            }
        }

        if !dirty {
            counter += 1;
        }
    }

    return counter;
}

fn part_two(phrases: Vec<String>) -> u32 {
    let mut counter = 0;

    for row in phrases {
        let words = row.split_whitespace();
        let mut phrases: HashSet<String> = HashSet::new();
        let mut dirty = false;

        for word in words {
            let mut cur_word: Vec<char> = word.chars().collect();
            cur_word.sort();

            let phrase:String = cur_word.into_iter().collect();

            if phrases.contains(&phrase) {
                dirty = true;
                break;
            } else {
                phrases.insert(phrase);
            }
        }

        if !dirty {
            counter += 1;
        }
    }

    return counter;
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let phrases = util::read_lines(filename);
    println!("PartOne: {}", part_one(phrases.clone()));

    println!("PartTwo: {}", part_two(phrases.clone()));
}
