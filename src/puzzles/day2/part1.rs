use std::convert::TryInto;
use crate::tools::tools;

pub fn run() {
    let input = tools::read_input("/Users/ejones/Documents/GitHub/advent-of-code-2025/src/puzzles/day2/puzzle_part1".to_string());

    let lines: Vec<&str> = input.lines().collect();

    let ranges: Vec<&str> = lines[0].split(",").collect();

    let mut duplicates: i128 = 0;
    for range in &ranges {
        let range_parts: Vec<&str> = range.split("-").collect();

        duplicates += calculate_duplicates(range_parts[0], range_parts[1]);
    }

    println!("Duplicates: {}", duplicates);
}

fn calculate_duplicates(min: &str, max: &str) -> i128 {
    let mut duplicates: i128 = 0;

    let minNum = min.parse::<i64>().unwrap();
    let maxNum = max.parse::<i64>().unwrap();

    for j in (minNum..maxNum+1).step_by(1) {
        let iString: String = j.to_string();

        if iString.len() % 2 != 0 {
            continue;
        }

        if iString[0..iString.len()/2] == iString[iString.len()/2..] {
            duplicates += j as i128;
        }
    }

    return duplicates;
}