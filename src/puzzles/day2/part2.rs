use std::convert::TryInto;
use crate::tools::tools;

pub fn run() {
    let input = tools::read_input("/Users/ejones/Documents/GitHub/advent-of-code-2025/src/puzzles/day2/puzzle_part1".to_string());

    let lines: Vec<&str> = input.lines().collect();

    let ranges: Vec<&str> = lines[0].split(",").collect();

    let mut duplicates: i64 = 0;
    for range in &ranges {
        let range_parts: Vec<&str> = range.split("-").collect();

        duplicates += calculate_duplicates(range_parts[0], range_parts[1]);
    }

    println!("Duplicates: {}", duplicates);
}

fn calculate_duplicates(min: &str, max: &str) -> i64 {
    let mut duplicates: i64 = 0;

    let minNum = min.parse::<i64>().unwrap();
    let maxNum = max.parse::<i64>().unwrap();

    for j in (minNum..maxNum+1).step_by(1) {
        let iString: String = j.to_string();

        let mut avoidDuplicates = Vec::new();

        for repeatLength in 1..((iString.len() as i64)/2) + 1 {
            if iString.len() as i64 % repeatLength != 0 {
                continue;
            }

            let repeatPattern = &iString[0..repeatLength as usize];

            let mut repeated = true;
            for m in (0..iString.len() as i64).step_by(repeatLength as usize) {
                if &iString[m as usize..(m+repeatLength) as usize] != repeatPattern {
                    repeated = false;
                    break;
                }
            }

            if repeated && avoidDuplicates.contains(&iString) == false {
                avoidDuplicates.push(iString.clone());
                duplicates += j;
            }
        }
    }

    return duplicates;
}